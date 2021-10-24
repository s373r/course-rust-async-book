// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 1.3. async/.await Primer

fn main() {
    // https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html
    println!("--- 1.3. async/.await Primer ---");
    {
        println!("\n--- A) simple example ---");
        {
            use futures::executor::block_on;

            async fn hello_world() {
                println!("hello, world!");
            }

            let future = hello_world(); // Nothing is printed
            block_on(future); // `future` is run and "hello, world!" is printed
        }

        // NOTE: Async functions have been implemented to run the following snippets: B, C

        use async_std::task;
        use std::time::Duration;

        #[derive(Debug)]
        struct Song {
            artist: &'static str,
            track: &'static str,
        }

        async fn learn_song() -> Song {
            let song = Song {
                artist: "Nirvana",
                track: "Smells Like Teen Spirit",
            };

            println!("> start learn_song: {:?}", song);

            // NOTE: we cannot use `std::thread::sleep()` since it is a blocking call
            task::sleep(Duration::from_millis(200)).await;

            // NOTE: `println!` is a blocking thing but we don't take that into account here
            println!("> finish learn_song: {:?}", song);

            return song;
        }

        async fn sing_song(song: Song) {
            println!("> start sing_song: {:?}", song);

            task::sleep(Duration::from_millis(100)).await;

            println!("> finish sing_song: {:?}", song);
        }

        async fn dance() {
            println!("> start dance");

            task::sleep(Duration::from_millis(100)).await;

            println!("> finish dance");
        }

        println!("\n--- B) ---");
        {
            use futures::executor::block_on;

            // NOTE: Async functions have been implemented above

            let song = block_on(learn_song());
            block_on(sing_song(song));
            block_on(dance());
        }

        println!("\n--- С) ---");
        {
            use futures::executor::block_on;

            // NOTE: Async functions have been implemented above

            async fn learn_and_sing() {
                // Wait until the song has been learned before singing it.
                // We use `.await` here rather than `block_on` to prevent blocking the
                // thread, which makes it possible to `dance` at the same time.
                let song = learn_song().await;
                sing_song(song).await;
            }

            async fn async_main() {
                let f1 = learn_and_sing();
                let f2 = dance();

                // `join!` is like `.await` but can wait for multiple futures concurrently.
                // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
                // future will take over the current thread. If `dance` becomes blocked,
                // `learn_and_sing` can take back over. If both futures are blocked, then
                // `async_main` is blocked and will yield to the executor.
                futures::join!(f1, f2);
            }

            block_on(async_main());
        }
    }
}
