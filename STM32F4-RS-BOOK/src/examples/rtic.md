# RTIC
So far, we have only used blocking logic. Meaning, we only do one thing at a time. So for example when we blink our LED, we rush over to the light switch and flip it, then religiously wait for one second. Meaning we do nothing while we wait, which seems like a waste doesn't it? 

Well, RTIC is here to save the day! Now we can boost our productivity by 1000^10%! Think about all the things you could do while waiting, like cleaning your room *side-eye*.


To use RTIC, we will need a couple more crates:
```rust
use rtt_target::{rprintln, rtt_init_print};
use panic_rtt_target as _;
use rtic::app;
use rtic_monotonics::systick::prelude::*;
```
* `use rtt_target::{rprintln, rtt_init_print};` - Imports the Real-Time Transfer (RTT) interface.
    * `rtt_init_print` - Print line macro that works of RTT.
    * `rprintln` - Used to initialize RTT. 
* `use panic_rtt_target as _;` - Imports panic handler for RTT.
* `use rtic::app;;` - Imports the RTIC app.
* `use rtic_monotonics::systick::prelude::*;` - Imports monotonic timer.


What is a monotonic timer? I am glad you asked! It it our reference used to schedule tasks and set delays. In the example below, we make a timer running at 1000 Hz, which equals about 1 ms resolution. Allowing us to schedule in milliseconds. 
```rust
systick_monotonic!(Mono, 1000); // 1000 Hz
```

Next, we will have a look at the structure of a RTIC app:
```rust
#[app(
    device = stm32f4xx_hal::pac,  // This device uses the stm32f4xx_hal Peripheral Access Crate (PAC).
    peripherals = true,           // Auto-initializes the Peripherals struct (dp).
    dispatchers = [SPI1],         // Unused interrupts that RTIC can use internally for software tasks, in this case SPI1. 
)]
mod app {
    // Import everything (*) from the parent module (rtic_blinky.rs)
    use super::*;

    // Resources
    #[shared] // Shared between different tasks
    struct Shared {}

    #[local] // Task local data only
    struct Local {}


    #[init] // Start-up function that initializes the program.
    fn init(cx: init::Context) -> (Shared, Local) {
        (...)
        (Shared {}, Local {})
    }
}
```


The first part of our structure has the `#[app()]`. It contains the configuration for our RTIC app, like which device we are using (STM32F4), if we want to auto-initialize our peripherals (`peripherals`) and which interrupts (`SPI1`) are free for the app to use (`dispatchers`). 
```rust
#[app(
    device = stm32f4xx_hal::pac,  
    peripherals = true,           
    dispatchers = [SPI1],         
)]
```

Next, we have the app body.
```rust
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init] 
    fn init(cx: init::Context) -> (Shared, Local) {
        (...)
        (Shared {}, Local {})
    }
}
```

The first line `use super::*;`, simply brings all (`*`) the imports from the module (`your_code.rs`) into the app. 


Next, we have the resources avaiable in the app, think variables, data structures etc.  
* The shared struct (`struct Shared {}`), contains resources shared between tasks, like sensor readings.
* The local struct (`struct Local {}`), contains local data only used within a task, and is NOT shared between structs. 
```rust
#[shared]
struct Shared {}

#[local]
struct Local {}
```


Lastly we have the `init` function. This functions initializes our program and kicks it off. We need it since we don't have a main loop anymore. 
```rust
#[init] 
fn init(cx: init::Context) -> (Shared, Local) {
    (...)
    (Shared {}, Local {})
}
```

The `init` function takes our `Local` and `Shared` struct, and puts it into the context struct `cx`.  
It also allows us to initialize our resources:
```rust
(Shared {}, Local {})
```

In addition, we have tasks `#[task()]`. We use these to perform actions, like blinking a LED, reading a UART message or setting the duty cycle for PWM. 
A task may look like this:
```rust
 #[task(local = [led, state])] // This task uses the local resources "led" and "state".
    async fn blink(cx: blink::Context) { // Use context cx to access local and shared resources.
        loop {
            rprintln!("blink");
            // Access local resources from context (cx.local)
            if *cx.local.state {         // If LED is on.
                cx.local.led.set_high();
                *cx.local.state = false;
            } else {                     // If LED is off
                cx.local.led.set_low();
                *cx.local.state = true;
            }
            // At the end of the task, wait 1000 ms (none-blocking).
            Mono::delay(1000.millis()).await;
        }
    }
```
* `#[task(local = [led, state])]` - Here the task uses the `led` and `state` resources from the local resources struct `Local`. 
* `#async fn blink(cx: blink::Context)` - We declare it as a async function (always required in RTIC V2), and access the resources and peripherals through context(`cx`). 

The loop operates similarlly as we have done before, but we now access the LED and its state through the `cx.local` struct.
```rust
loop {
    rprintln!("blink");
    // Access local resources from context (cx.local)
    if *cx.local.state {         // If LED is on.
        cx.local.led.set_high();
        *cx.local.state = false;
        ...
    }
}
```

Finally at the end, we make use of our monotonic timer:
```rust
Mono::delay(1000.millis()).await;
```

As opposed to our old delay, this delay will not block the CPU, allowing it to do other tasks in the background while the `blink` task is waiting.