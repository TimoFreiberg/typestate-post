package repairorder.singlevariables

import Customer
import Employee
import findIdleTechnician

fun process(order: RepairOrder) {
    if (order.isValid()) {
        order.valid = true
    } else {
        order.valid = false
        order.validationErrors = order.validationErrors()
        storeInvalidOrder(order)
        return
    }
    assert(order.valid == true)

    val technician = findIdleTechnician()
    order.assignedTechnician = technician

    order.inProgress = true

    order.stepsLeft = calculateSteps(order)
    while (order.stepsLeft.isNotEmpty()) {
        workOnNextStep(order)
    }
    assert(order.stepsLeft.isEmpty())

    val invoice = sendInvoice(order)
    order.invoice = invoice

    awaitPayment()
    order.paid = true

    assert(order.paid)
}

data class RepairOrder(
        val orderNumber: Long,
        val damageDescription: String?,
        val vehicle: String,
        val customer: Customer,
        var valid: Boolean? = null,
        var validationErrors: List<String> = emptyList(),
        var assignedTechnician: Employee? = null,
        var inProgress: Boolean = false,
        var stepsLeft: MutableList<String> = mutableListOf(),
        var paid: Boolean = false,
        var invoice: String? = null,
)

fun awaitPayment(): Boolean {
    TODO()
}

fun sendInvoice(order: RepairOrder): String {
    TODO()
}

fun workOnNextStep(order: RepairOrder) {
    TODO()
}

fun calculateSteps(order: RepairOrder): MutableList<String> {
    TODO()
}

private fun RepairOrder.isValid(): Boolean {
    TODO()
}

fun RepairOrder.validationErrors(): List<String> {
    TODO()
}

fun storeInvalidOrder(order: RepairOrder) {
    TODO()
}

