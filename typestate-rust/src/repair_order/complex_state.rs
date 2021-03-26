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
        OneOf4::A(invalid) => process_invalid(invalid),
        OneOf4::B(low_prio) => {
            todo!()
        }
        OneOf4::C(high_prio) => {
            todo!()
        }
        OneOf4::D(april_fools) => {
            todo!()
        }
    }
}

fn process_invalid(invalid: RepairOrder<Invalid>) -> EndStates {
    match invalid.recover() {
        Ok(recovered) => {
            let waiting = match recovered.prioritize().enqueue() {
                Ok(waiting) => waiting,
                Err(high_prio) => high_prio.enqueue(),
            };
            todo!()
        }
        Err(krangled) => OneOf4::D(krangled),
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
    fn prioritize(self) -> RepairOrder<LowPriority> {
        self.with_state(LowPriority)
    }
}

impl RepairOrder<LowPriority> {
    fn enqueue(self) -> RepairOrderResult<WaitingForWorker, HighPriority> {
        if self.order_number % 3 == 0 {
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
        if self.order_number < 1000 {
            Ok(self.with_state(Recovered))
        } else {
            Err(self.with_state(Krangled))
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
