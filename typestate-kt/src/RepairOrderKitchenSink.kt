fun process(order: RepairOrder) {
    if (order.validate()) {
        order.valid = true
    } else {
        order.valid = false
        order.validationErrors = order.validationErrors()
        storeInvalidOrder(order)
        return
    }

    assert(order.valid == true)

    while (order.assignedTechnician == null) {
        when (val technician = findIdleTechnician()) {
            null -> Thread.sleep(1000 * 60 * 30)
            else -> order.assignedTechnician = technician
        }
    }

    assert(order.assignedTechnician != null)

    order.inProgress = true

    // TODO add steps, add a way to block, add inventory handling (blocked for items/part of steps), add payment handling
}


data class RepairOrder(
        val orderNumber: Long,
        val isInspection: Boolean,
        val damageDescription: String?,
        val vehicle: String,
        val customer: Customer,
        var valid: Boolean? = null,
        var validationErrors: List<String> = emptyList(),
        var assignedTechnician: Employee? = null,
        var inProgress: Boolean = false,
        val stepsLeft: List<String> = emptyList(),
        var blocked: Boolean = false,
        val waitingForItems: List<String> = emptyList(),
        var paid: Boolean = false,
        var invoice: String? = null,
)

private fun RepairOrder.validate(): Boolean = !customer.hasOutstandingDebt && !customer.isBanned

fun RepairOrder.validationErrors(): List<String> {
    val errors = mutableListOf<String>()
    if (customer.hasOutstandingDebt) {
        errors.add("Customer has outstanding debt")
    }
    if (customer.isBanned) {
        errors.add("Customer is banned from the shop")
    }
    return errors
}

fun storeInvalidOrder(order: RepairOrder) {
    println("Please store me: $order")
}

fun findIdleTechnician() = Employee()
