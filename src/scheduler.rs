/*
 * States
 * 
 * COLD
 * WARMING
 * RUNNING
 * STOPPING
 * STOPPED
 *
 */

pub struct Event {
    id: u32,
    state: String,
    description: String,
    data: Json,
}
