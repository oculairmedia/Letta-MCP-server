//! Simple test environment documentation
//! 
//! For environment variable testing, we use #[serial_test::serial] to ensure tests
//! that manipulate environment variables run sequentially to avoid conflicts.
//! 
//! This is the cleanest approach without complex abstractions or unsafe code.
