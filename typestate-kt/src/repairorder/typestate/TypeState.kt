package repairorder.typestate

import Customer
import Employee
import findIdleTechnician

fun process(order: RepairOrder<New>) {
    assert(order.state == New)

    val valid = order.validate().unwrapOrElse { return }

    val technician = findIdleTechnician()
    val stepsLeft = calculateSteps()
    val inProgress = valid.startProgress(technician, stepsLeft)

    val done = inProgress.work()

    val waitingForPayment = done.sendInvoice()
    val paid = waitingForPayment.awaitPayment()
}

data class RepairOrder<T>(
        val orderNumber: Long,
        val damageDescription: String?,
        val vehicle: String,
        val customer: Customer,
        val state: T
)

fun <T, T2> RepairOrder<T>.withState(state: T2): RepairOrder<T2> = RepairOrder(
        orderNumber, damageDescription, vehicle, customer, state
)

object New
object Valid
data class Invalid(val validationErrors: List<String>)
data class InProgress(val assignedTechnician: Employee, val stepsLeft: MutableList<String>)
object WorkDone
data class WaitingForPayment(val invoice: String)
data class Paid(val invoice: String)

private fun RepairOrder<New>.validate(): Result<RepairOrder<Valid>, RepairOrder<Invalid>> {
    val isValid = isValid()
    return if (isValid) {
        Result.Ok(this.withState(Valid))
    } else {
        val validationErrors = getValidationErrors()
        Result.Err(this.withState(Invalid(validationErrors)))
    }
}

private fun RepairOrder<Valid>.startProgress(technician: Employee, stepsLeft: MutableList<String>) =
        withState(InProgress(technician, stepsLeft))

internal fun RepairOrder<InProgress>.work(): RepairOrder<WorkDone> {
    while (state.stepsLeft.isNotEmpty()) {
        this.workOnNextStep()
    }
    return withState(WorkDone)
}

fun RepairOrder<WorkDone>.sendInvoice(): RepairOrder<WaitingForPayment> {
    val invoice = getInvoice()
    return withState(WaitingForPayment(invoice))
}

private fun RepairOrder<WaitingForPayment>.awaitPayment(): RepairOrder<Paid> {
    val invoice = state.invoice

    repairorder.typestate.awaitPayment()

    return withState(Paid(invoice))
}

private fun RepairOrder<InProgress>.workOnNextStep() {
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

sealed class Result<T, E> {
    data class Ok<T, E>(val v: T) : Result<T, E>()
    data class Err<T, E>(val e: E) : Result<T, E>()
}

inline fun <T,E> Result<T,E>.unwrapOrElse(orElse: (E) -> T): T = when (this) {
    is Result.Ok -> this.v
    is Result.Err -> orElse(this.e)
}