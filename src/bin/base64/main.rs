

pub fn main() {
    let output1 = base64::decode_config(
        "OQ58euPBcuWWxdRd4SmxxUl-vzGzcdAvRMmPOuVFRdwLxWDtOZK9vD2jefUtxfUqCfLamSpjOWUP4qcjPfG5QSBFYqUJrgmLfdBZQfAExoHbcfyAHWp6HS5YPImZxgcZvgDjOdwzmMpCWYFwQDUyfqmeRSPBQNN9etZXQNR9CuPpRqw0xQRgvqKbvf96PDLAHZW8OUfwPD5aVQUd3YmX3fF6QuHTPzWsedo0vBHA4Ymtmje9xNAgegLlvdWyxjW54I55RW5JfD5jHZRxRIWKxqot4gPQedmIOdwdVBGwQoajRWLXrU567QWwYgpeRuHpf-pXQSWZWzfZOBLPWfDwVBUDvBHtWZRSHjad7NZurgc-f-5mvu7Z7QpsOjf-cWWpWYoq3B5yYWLvQdPhWWcw4SLwxYGJRqGxPoobPQHMxQUPQSWxQgpfRjUQRMLCRMK7vfoCfMRHPIoUPNZ9OZWMvYWoYBm8P-msQ-mbmWoLebZlWg5gcBPHxq7jv-oavzNZRq6uOSH3PUopvjL-WMijQvin",
        base64::URL_SAFE).unwrap();

    let output2 = base64::decode_config(
"PjcZRYw6cdFtHBwvVYHaQDGV7YF-7f2dxNcNRgH9OdwbOYDNcqljOj5kQBHjCQRhcBG84QHH7f6qfdmFPkZFWjmjVSK8fDGKrqNdPul-VtZBfWLsrUL8fI5V7QWErqoWmM5Fff6tHg5-czm-vDoX3IW3Yg5YHqU57QUyvumgYBo8ODGCmz9trMRpYg7w7DWfRBHlR-HmOMWerUotHYPDeYnZPzy6RuLZ7YP8VupNRdmo4DWyVNGyQWppHQpERILKPgUZPqKSPMomVQUNxQo5QZopPQp3OkNNRQ5X4WPNfDRbmWox4vZ5cQDwmSoLODPWxSAvmuUumUR8PuppOzy97fPbxQHpmfWUfSWaeQPlH-L7fZp-HSHa4Qi-xdUYvYKXvQW6QBGKPZDtYImKCQL33zRTHDHQWzndRgeAmIWxRfZMcMcFvf6ZxzLyvqZP3SPu3WeAQSAmvYAAVUouvopH4gm63zZDvNn-3fF6YUBFmIp84Ip8HoPSPuUBQN9FVf5KxSmYWfUxQIPVWuHkxgp5vtin",
            base64::URL_SAFE).unwrap();


    let output3 = base64::decode_config(
        "PoKBxZHtfDKjVM5mxtNF3I5IegLQO-pKvf6SfYPbvNnFcDGbYW5Ufg5MPSmYRWpNWWHFWzVdcqU6W-WImzRA3ILKHUHS3fHZVdwPHWofONLlVq6oVWHPcfK0RzPY7W56RzA5CYUMvzmHfYZaPW5yVMLucQR6YBKSYDU9rqwoxqAUHg5lCY6f7qmzVBH7RfwufqwP4qc9Vqm8VD6a3BWbQq5ZcBHZYuK07jWMWUKjvBAJvMULf-RbYYw-OBfAeUH6HBUpOqAhxI5hRDVAeWUVfg57OqUUcZm8mqUVPIm9fq5m4g5qVZKYmDUs4SKfVfBtvoKwQN6CcZ5eCQUL4f5trU5AfD7FPgLAQzHJfMWoWIlNPYLXeDLLcqGgCYnwYgmDPBajfDBjmZU0WDejmNKzHUPaRILvcWPoWzfNYBGw7u5TP-5yRYZAYI79xBoJfWUDQjKqVYUevjm0WzwdcupA3fwLxWiNPzmuOMKK7M5BvfPxPuoCPQUvvYmDQzPYOzGKYzRMfzPMQgLpVYKNVgRf4bin",
                    base64::URL_SAFE).unwrap();

    println!("{}", hex::encode(output1));
    println!("{}", hex::encode(output2));
    println!("{}", hex::encode(output3));
}