package repairorder.stateenum

import Customer
import Employee
import org.junit.jupiter.api.Test
import repairorder.typestate.RepairOrder
import repairorder.typestate.State

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
        repairorder.typestate.work()
    }
}