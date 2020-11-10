package repairorder.stateenum

import Customer
import Employee
import org.junit.jupiter.api.Test

class StateEnumKtTest {
    @Test
    fun `mutate state under work fun`() {
        val repairOrder = RepairOrder(
                0,
                "description",
                "vehicle",
                Customer(
                        false,
                        false
                ),
                state = State.InProgress(Employee(), mutableListOf("A", "B"))
        )
        repairOrder.work()
    }
}