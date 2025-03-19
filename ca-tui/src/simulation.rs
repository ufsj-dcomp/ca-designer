use std::{sync::Arc, time::Duration};

use libca::simulation::SimulationContext;
use tokio::{
    select,
    sync::{
        mpsc::{Receiver, Sender},
        Mutex,
    },
    time::sleep,
};

#[derive(Debug)]
pub enum ThreadCommand {
    Resume,
    Pause,
    Forward,
    // UpdateModel(libca::Model),
    SetGridItem {
        x: usize,
        y: usize,
        state: libca::NodeId,
    },
}

#[derive(Debug)]
pub struct ThreadOutput;

const SIMULATION_DELTA: Duration = Duration::from_millis(500);

pub async fn simulation_thread(
    ctx: Arc<Mutex<SimulationContext>>,
    mut rx: Receiver<ThreadCommand>,
    tx: Sender<()>,
) {
    let is_running = true;

    loop {
        if is_running {
            {
                let mut simulation = ctx.lock().await;
                simulation.step();
            }
            let _ = tx.send(()).await;

            select! {
                cmd = rx.recv() => {}
                _ = sleep(SIMULATION_DELTA) => {}
            }
        } else {
            let cmd = rx.recv().await;
        }
    }
}
