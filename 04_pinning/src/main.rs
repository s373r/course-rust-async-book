// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 4. Pinning

fn main() {
    // https://rust-lang.github.io/async-book/04_pinning/01_chapter.html
    println!("--- 4. Pinning ---");
    {
        // https://rust-lang.github.io/async-book/04_pinning/01_chapter.html#pinning-in-detail
        println!("\n--- Pinning in Detail ---");
        {
            // NOTE: unused import
            // use std::pin::Pin;

            #[derive(Debug)]
            struct Test {
                a: String,
                b: *const String,
            }

            impl Test {
                fn new(txt: &str) -> Self {
                    Test {
                        a: String::from(txt),
                        b: std::ptr::null(),
                    }
                }

                fn init(&mut self) {
                    let self_ref: *const String = &self.a;
                    self.b = self_ref;
                }

                fn a(&self) -> &str {
                    &self.a
                }

                fn b(&self) -> &String {
                    assert!(
                        !self.b.is_null(),
                        "Test::b called without Test::init being called first"
                    );
                    unsafe { &*(self.b) }
                }
            }

            println!("\n--- A) ---");
            {
                let mut test1 = Test::new("test1");
                test1.init();
                let mut test2 = Test::new("test2");
                test2.init();

                println!("a: {}, b: {}", test1.a(), test1.b()); // a: test1, b: test1
                println!("a: {}, b: {}", test2.a(), test2.b()); // a: test2, b: test2
            }

            println!("\n--- B) ---");
            {
                let mut test1 = Test::new("test1");
                test1.init();
                let mut test2 = Test::new("test2");
                test2.init();

                println!("a: {}, b: {}", test1.a(), test1.b()); // a: test1, b: test1
                std::mem::swap(&mut test1, &mut test2);
                println!("a: {}, b: {}", test2.a(), test2.b()); // a: test1, b: test2
            }

            println!("\n--- C) ---");
            {
                let mut test1 = Test::new("test1");
                test1.init();
                let mut test2 = Test::new("test2");
                test2.init();

                println!("a: {}, b: {}", test1.a(), test1.b()); // a: test1, b: test1
                std::mem::swap(&mut test1, &mut test2);
                test1.a = "I've totally changed now!".to_string();
                println!("a: {}, b: {}", test2.a(), test2.b()); // a: test1, b: I've totally
                                                                //                                                              changed now!
            }
        }

        // https://rust-lang.github.io/async-book/04_pinning/01_chapter.html#pinning-to-the-stack
        println!("\n--- Pinning to the Stack ---");
        {
            use std::marker::PhantomPinned;
            use std::pin::Pin;

            #[derive(Debug)]
            struct Test {
                a: String,
                b: *const String,
                _marker: PhantomPinned,
            }

            impl Test {
                fn new(txt: &str) -> Self {
                    Test {
                        a: String::from(txt),
                        b: std::ptr::null(),
                        _marker: PhantomPinned, // This makes our type `!Unpin`
                    }
                }

                fn init(self: Pin<&mut Self>) {
                    let self_ptr: *const String = &self.a;
                    let this = unsafe { self.get_unchecked_mut() };
                    this.b = self_ptr;
                }

                fn a(self: Pin<&Self>) -> &str {
                    &self.get_ref().a
                }

                fn b(self: Pin<&Self>) -> &String {
                    assert!(
                        !self.b.is_null(),
                        "Test::b called without Test::init being called first"
                    );
                    unsafe { &*(self.b) }
                }
            }

            println!("\n--- A) ---");
            {
                // test1 is safe to move before we initialize it
                let mut test1 = Test::new("test1");
                // Notice how we shadow `test1` to prevent it from being accessed again
                let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
                Test::init(test1.as_mut());

                let mut test2 = Test::new("test2");
                let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
                Test::init(test2.as_mut());

                println!(
                    "a: {}, b: {}",
                    Test::a(test1.as_ref()),
                    Test::b(test1.as_ref())
                );
                println!(
                    "a: {}, b: {}",
                    Test::a(test2.as_ref()),
                    Test::b(test2.as_ref())
                );
            }

            println!("\n--- B) ---");
            {
                let mut test1 = Test::new("test1");
                let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
                Test::init(test1.as_mut());

                let mut test2 = Test::new("test2");
                let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
                Test::init(test2.as_mut());

                println!(
                    "a: {}, b: {}",
                    Test::a(test1.as_ref()),
                    Test::b(test1.as_ref())
                );
                // NOTE: commented to continue execution
                // std::mem::swap(test1.get_mut(), test2.get_mut());
                println!(
                    "a: {}, b: {}",
                    Test::a(test2.as_ref()),
                    Test::b(test2.as_ref())
                );
            }

            println!("\n--- C) ---");
            {
                let mut test1 = Test::new("test1");
                let mut test1_pin = unsafe { Pin::new_unchecked(&mut test1) };
                Test::init(test1_pin.as_mut());

                drop(test1_pin);
                println!(r#"test1.b points to "test1": {:?}..."#, test1.b);

                let mut test2 = Test::new("test2");
                std::mem::swap(&mut test1, &mut test2);
                println!("... and now it points nowhere: {:?}", test1.b);

                // NOTE: print `test1` & `test2`
                println!("test1: {:?}", test1);
                println!("test2: {:?}", test2);
            }
        }

        // https://rust-lang.github.io/async-book/04_pinning/01_chapter.html#pinning-to-the-stack
        println!("\n--- Pinning to the Heap ---");
        {
            use std::marker::PhantomPinned;
            use std::pin::Pin;

            #[derive(Debug)]
            struct Test {
                a: String,
                b: *const String,
                _marker: PhantomPinned,
            }

            impl Test {
                fn new(txt: &str) -> Pin<Box<Self>> {
                    let t = Test {
                        a: String::from(txt),
                        b: std::ptr::null(),
                        _marker: PhantomPinned,
                    };
                    let mut boxed = Box::pin(t);
                    let self_ptr: *const String = &boxed.as_ref().a;
                    unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };

                    boxed
                }

                fn a(self: Pin<&Self>) -> &str {
                    &self.get_ref().a
                }

                fn b(self: Pin<&Self>) -> &String {
                    unsafe { &*(self.b) }
                }
            }

            let test1 = Test::new("test1");
            let test2 = Test::new("test2");

            println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
            println!("a: {}, b: {}", test2.as_ref().a(), test2.as_ref().b());
        }
    }
}
