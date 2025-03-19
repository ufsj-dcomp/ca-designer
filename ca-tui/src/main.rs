#![feature(iter_intersperse)]
#![feature(if_let_guard)]
#![feature(iter_map_windows)]

mod app;
pub mod simulation;
pub mod tab;
pub mod widgets;

use std::sync::Arc;

use app::{App, Message};
use crossterm::event::{self, EventStream, KeyEvent, KeyEventKind};
use futures::StreamExt;
use libca::{
    grid::{neighbor_strategy::NeighboringStrategy, Grid, StateProbabilty},
    simulation::SimulationContext,
    Model, NodeId,
};
use simulation::ThreadCommand;
use tokio::{select, sync::Mutex};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let (cmd_tx, cmd_rx) = tokio::sync::mpsc::channel(10);
    let (sim_tx, mut sim_rx) = tokio::sync::mpsc::channel(1);

    let (width, height) = crossterm::terminal::size()?;

    let mut grid = Grid::empty(
        (width * height) as usize,
        width as usize,
        NeighboringStrategy::SquareAndCorners,
    );
    grid.randomize(&[
        StateProbabilty {
            state: NodeId::from_index(0),
            weight: 1.0,
        },
        StateProbabilty {
            state: NodeId::from_index(1),
            weight: 0.3,
        },
    ])?;

    let simulation_ctx = Arc::new(Mutex::new(SimulationContext::new(
        Model::game_of_life(),
        grid,
    )));

    let task = tokio::spawn(simulation::simulation_thread(
        Arc::clone(&simulation_ctx),
        cmd_rx,
        sim_tx,
    ));

    let mut app = App::new(simulation_ctx);
    let mut reader = EventStream::new();
    let mut should_quit = false;

    while !should_quit {
        app.draw_to(&mut terminal).await?;

        let read_event = async {
            if let Some(Ok(event::Event::Key(
                key_ev @ KeyEvent {
                    kind: KeyEventKind::Press,
                    ..
                },
            ))) = reader.next().await
            {
                match app.handle_key_press(key_ev).await {
                    Message::ResumeSimulation => {
                        let _ = cmd_tx.send(ThreadCommand::Resume).await;
                    }
                    Message::PauseSimulation => {
                        let _ = cmd_tx.send(ThreadCommand::Pause).await;
                    }
                    Message::StepSimulation => {
                        let _ = cmd_tx.send(ThreadCommand::Forward).await;
                    }
                    Message::UpdateModel(model) => todo!(),
                    Message::CloseApplication => should_quit = true,
                    Message::None => {}
                }
            };
        };

        select! {
            _ = read_event => {},
            _ = sim_rx.recv() => {},
        };
    }

    task.abort();
    Ok(())
}
