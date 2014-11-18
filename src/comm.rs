use std::comm::{Sender, Receiver};
use std::cmp::max;

pub struct DuplexChannel<T: Send> {
    top_sender: Option<Sender<T>>,
    top_receiver: Option<Receiver<T>>,
    bottom_sender: Option<Sender<T>>,
    bottom_receiver: Option<Receiver<T>>
}

impl<T: Send> DuplexChannel<T> {
    fn new() -> DuplexChannel<T> {
        DuplexChannel {
            top_sender: None,
            top_receiver: None,
            bottom_sender: None,
            bottom_receiver: None
        }
    }
    
    pub fn get_chain(number: uint) -> Vec<DuplexChannel<T>> {    
        let mut result_vector = Vec::from_fn(number, |_| DuplexChannel::new());
        
        for i in range(0, max(1, number) - 1) {
            let (bottom_sender, top_receiver) = channel::<T>();
            let (top_sender, bottom_receiver) = channel::<T>();
            
            result_vector[i].bottom_sender = Some(bottom_sender);
            result_vector[i].bottom_receiver = Some(bottom_receiver);
            result_vector[i+1].top_sender = Some(top_sender);
            result_vector[i+1].top_receiver = Some(top_receiver);
        }
        
        result_vector
    }
    
    pub fn send_top(&self, value: T) {
        match self.top_sender {
            Some(ref x) => x.send(value),
            None    => ()
        }
    }
    
    pub fn receive_top(&self) -> Option<T> {
        match self.top_receiver {
            Some(ref x) => Some(x.recv()),
            None    => None
        }
    }
    
    pub fn send_bottom(&self, value: T) {
        match self.bottom_sender {
            Some(ref x) => x.send(value),
            None    => ()
        }
    }
    
    pub fn receive_bottom(&self) -> Option<T> {
        match self.bottom_receiver {
            Some(ref x) => Some(x.recv()),
            None    => None
        }
    }
}

