data class RepairOrder(
    val orderNumber: Long,k
    var valid: Boolean? = null,
    var validationErrors: MutableList<String> = mutableListOf(),
    var inProgress: Boolean = false,
    var stepsLeft: MutableList<String> = mutableListOf(),
    var blocked: Boolean = false,
    var waitingForItems: MutableList<String> = mutableListOf(),
    var paid: Boolean = false,
    var invoice: String? = null
)

fun RepairOrder.validate() {
    if (Math.random() < 0.2) {
        valid = false
        validationErrors.add("This order is out of order")
    } else {
        valid = true
    }
}
