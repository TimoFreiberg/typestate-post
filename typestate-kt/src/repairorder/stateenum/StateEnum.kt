package repairorder.stateenum

import Customer
import Employee
import findIdleTechnician

fun process(order: RepairOrder) {
    assert(order.state == State.New)

    order.validate()
    if (order.state is State.Invalid) {
        return
    }

    val technician = findIdleTechnician()
    val stepsLeft = calculateSteps()
    order.startProgress(technician, stepsLeft)

    order.work()

    order.sendInvoice()
    order.awaitPayment()
}

data class RepairOrder(
        val orderNumber: Long,
        val damageDescription: String?,
        val vehicle: String,
        val customer: Customer,
        var state: State = State.New,
)

sealed class State {
    object New : State()
    object Valid : State()
    data class Invalid(val validationErrors: List<String>) : State()
    data class InProgress(val assignedTechnician: Employee, val stepsLeft: MutableList<String>) : State()
    object WorkDone : State()
    data class WaitingForPayment(val invoice: String) : State()
    data class Paid(val invoice: String) : State()
}

private fun RepairOrder.validate() {
    val isValid = isValid()
    state = if (isValid) {
        State.Valid
    } else {
        val validationErrors = getValidationErrors()
        State.Invalid(validationErrors)
    }
}

private fun RepairOrder.startProgress(technician: Employee, stepsLeft: MutableList<String>) {
    assert(state is State.Valid)
    state = State.InProgress(technician, stepsLeft)
}

internal fun RepairOrder.work() {
    assert(state is State.InProgress)

    val stepsLeft = (state as State.InProgress).stepsLeft
    while (stepsLeft.isNotEmpty()) {
        this.workOnNextStep()
    }
    state = State.WorkDone
}

fun RepairOrder.sendInvoice() {
    val invoice = getInvoice()
    state = State.WaitingForPayment(invoice)
}

private fun RepairOrder.awaitPayment() {
    assert(state is State.WaitingForPayment)
    val invoice = (state as State.WaitingForPayment).invoice

    repairorder.stateenum.awaitPayment()

    state = State.Paid(invoice)
}

private fun RepairOrder.workOnNextStep() {
    TODO()
}

private fun calculateSteps(): MutableList<String> {
    TODO()
}

private fun isValid(): Boolean {
    TODO()
}

private fun getValidationErrors(): List<String> {
    TODO()
}

private fun getInvoice(): String {
    TODO()
}

private fun awaitPayment() {
    TODO()
}
