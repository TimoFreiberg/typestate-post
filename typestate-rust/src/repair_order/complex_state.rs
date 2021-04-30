use super::typestate::RepairOrder;

type EndStates = OneOf4<
    RepairOrder<Finished>,
    RepairOrder<AprilFools>,
    RepairOrder<Garbage>,
    RepairOrder<Krangled>,
>;
type RepairOrderResult<T1, T2> = Result<RepairOrder<T1>, RepairOrder<T2>>;

fn process(order: RepairOrder<New>) -> EndStates {
    match order.validate() {
        OneOf4::A(invalid) => match invalid.recover() {
            Ok(recovered) => match recovered.prioritize() {
                Ok(low_prio) => handle_low_prio(low_prio),
                Err(new) => process(new),
            },
            Err(krangled) => OneOf4::D(krangled),
        },
        OneOf4::B(low_prio) => match low_prio.enqueue() {
            Ok(waiting_for_worker) => {
                todo!()
            }
            Err(high_prio) => {
                todo!()
            }
        },
        OneOf4::C(high_prio) => {
            todo!()
        }
        OneOf4::D(april_fools) => {
            todo!()
        }
    }
}

fn handle_low_prio(low_prio: RepairOrder<LowPriority>) -> EndStates {
    match low_prio.enqueue() {
        Ok(waiting_for_worker) => match waiting_for_worker.send_print_job() {
            Ok(waiting_for_printer) => {
                todo!()
            }
            Err(garbage) => {
                todo!()
            }
        },
        Err(high_prio) => {
            todo!()
        }
    }
}

fn process_invalid(invalid: RepairOrder<Invalid>) -> EndStates {
    match invalid.recover() {
        Ok(recovered) => match recovered.prioritize() {
            Ok(low_prio) => handle_low_prio(low_prio),
            Err(new) => process(new),
        },
        Err(krangled) => OneOf4::D(krangled),
    }
}

impl<T> RepairOrder<T> {
    fn check(&self) -> bool {
        self.order_number % 2 == 0
    }
}

impl RepairOrder<New> {
    fn validate(
        self,
    ) -> OneOf4<
        RepairOrder<Invalid>,
        RepairOrder<LowPriority>,
        RepairOrder<HighPriority>,
        RepairOrder<AprilFools>,
    > {
        use OneOf4::*;
        match self.order_number % 4 {
            0 => A(self.with_state(Invalid)),
            1 => B(self.with_state(LowPriority)),
            2 => C(self.with_state(HighPriority)),
            3 => D(self.with_state(AprilFools)),
            _ => unreachable!(),
        }
    }
}

impl RepairOrder<Recovered> {
    fn prioritize(self) -> RepairOrderResult<LowPriority, New> {
        if self.check() {
            Ok(self.with_state(LowPriority))
        } else {
            Err(self.with_state(New))
        }
    }
}

impl RepairOrder<LowPriority> {
    fn enqueue(self) -> RepairOrderResult<WaitingForWorker, HighPriority> {
        if self.check() {
            Ok(self.with_state(WaitingForWorker))
        } else {
            Err(self.with_state(HighPriority))
        }
    }
}

impl RepairOrder<HighPriority> {
    fn enqueue(self) -> RepairOrder<WaitingForWorker> {
        self.with_state(WaitingForWorker)
    }
}

impl RepairOrder<Invalid> {
    fn recover(self) -> RepairOrderResult<Recovered, Krangled> {
        if self.check() {
            Ok(self.with_state(Recovered))
        } else {
            Err(self.with_state(Krangled))
        }
    }
}

impl RepairOrder<WaitingForWorker> {
    fn send_print_job(self) -> RepairOrderResult<WaitingForPrinter, Garbage> {
        if self.check() {
            Ok(self.with_state(WaitingForPrinter))
        } else {
            Err(self.with_state(Garbage))
        }
    }
}

enum OneOf4<A, B, C, D> {
    A(A),
    B(B),
    C(C),
    D(D),
}

struct New;
struct Invalid;
struct Recovered;
struct AprilFools;
struct LowPriority;
struct HighPriority;
struct WaitingForWorker;
struct Garbage;
struct WaitingForPrinter;
struct WaitingForFridayAfternoon;
struct WorkInProgress;
struct Krangled;
struct Finished;
