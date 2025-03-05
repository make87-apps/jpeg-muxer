use std::sync::Arc;
use make87_messages::image::compressed::ImageJpeg;

fn main() {
    make87::initialize();

    let publisher = {
        let topic_name = "OUTGOING_MESSAGE";
        match make87::resolve_topic_name(topic_name) {
            Some(topic_name) => {
                if let Some(topic) = make87::get_publisher::<ImageJpeg>(topic_name.clone()) {
                    Arc::new(topic)
                } else {
                    panic!("Failed to get publisher for topic '{}'", topic_name);
                }
            }
            None => {
                panic!("Failed to resolve topic name '{}'", topic_name);
            }
        }
    };

    for i in 0..10 {
        let topic_name = format!("INCOMING_MESSAGE_{}", i);
        let publisher = Arc::clone(&publisher);
        match make87::resolve_topic_name(&topic_name) {
            Some(resolved_topic_name) => {
                if let Some(topic) = make87::get_subscriber::<ImageJpeg>(resolved_topic_name) {
                    topic.subscribe(move |message| {
                        println!("Received message '{:?}'", message);
                        match publisher.publish(&message) {
                            Ok(()) => println!("Published: {:?}", &message),
                            Err(_) => eprintln!("Failed to publish: {:?}", &message),
                        }
                    }).unwrap();
                }
            }
            None => {
                panic!("Failed to resolve topic name '{}'", topic_name);
            }
        }
    }

    make87::keep_running();
}
