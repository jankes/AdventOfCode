package jankes.adventofcode

import java.io.File

fun main(args: Array<String>) {
    val wires = HashMap<String, Wire>()

    val input = File("C:\\Users\\sjank\\Documents\\Projects\\AdventOfCode\\2015\\7\\input.txt")
    input.forEachLine {
        val parts = it.split(' ')
        val gate: Gate
        if (it.contains("AND")) {
            val (left, right, result) = parseTwoInput(parts, wires)
            gate = TwoInputGate(And, left, right, result)
        } else if (it.contains("OR")) {
            val (left, right, result) = parseTwoInput(parts, wires)
            gate = TwoInputGate(Or, left, right, result)
        } else if (it.contains("RSHIFT")) {
            val (left, right, result) = parseTwoInput(parts, wires)
            gate = TwoInputGate(RShift, left, right, result)
        } else if (it.contains("LSHIFT")) {
            val (left, right, result) = parseTwoInput(parts, wires)
            gate = TwoInputGate(LShift, left, right, result)
        } else if (it.contains("NOT")) {
            gate = parseNot(parts, wires)
        } else {
            gate = parseDirectSet(parts, wires)
        }
        //println("$gate")

        gate.connectWires()
        gate.updateValue()
    }

    for (e in wires.entries.sortedBy({e -> e.key})) {
        println("${e.key} -> ${e.value}")
    }
}

fun parseTwoInput(parts: List<String>, wires: HashMap<String, Wire>): TwoInputGateData {
    val left = parseInput(parts[0], wires)
    val right = parseInput(parts[2], wires)
    val result = parseWire(parts[4], wires)
    return TwoInputGateData(left, right, result)
}

fun parseNot(parts: List<String>, wires: HashMap<String, Wire>): Gate {
    val left = parseInput(parts[1], wires)
    val result = parseWire(parts[3], wires)
    return OneInputGate(Not, left, result)
}

fun parseDirectSet(parts: List<String>, wires: HashMap<String, Wire>): Gate {
    val left = parseInput(parts[0], wires)
    val right = Input.Literal(0)
    val result = parseWire(parts[2], wires)
    return TwoInputGate(Or, left, right, result)
}

fun parseInput(str: String, wires: HashMap<String, Wire>): Input {
    if (str.startsWithNumber()) {
        return Input.Literal(str.toInt().toShort())
    } else {
        val wire = parseWire(str, wires)
        return Input.Wire(wire)
    }
}

fun parseWire(str: String, wires: HashMap<String, Wire>): Wire {
    val wire = wires[str]
    return when (wire) {
        is Wire -> wire
        else -> {
            val newWire = Wire(str)
            wires.put(str, newWire)
            newWire
        }
    }
}

fun String.startsWithNumber(): Boolean {
    val start = this.elementAt(0)
    return '0' <= start && start <= '9'
}

sealed class Input {
    data class Literal(val value: Short) : Input() {
        override fun toString(): String = value.toString()
    }

    data class Wire(val wire: jankes.adventofcode.Wire) : Input() {
        override fun toString(): String = wire.name
    }
}

fun Input.hasValue(): Boolean {
    return when (this) {
        is Input.Literal -> true
        is Input.Wire -> this.wire.hasValue
    }
}

fun Input.value(): Short {
    return when (this) {
        is Input.Literal -> this.value
        is Input.Wire -> wire.value
    }
}

class Wire(val name: String) {
    var hasValue = false
    var value: Short = 0
    val gates = mutableListOf<Gate>()

    override fun toString() = if (hasValue) {
        "($name: $value)"
    } else {
        "($name)"
    }
}

interface Gate {
    fun connectWires()
    fun updateValue()
}

data class TwoInputGateData(val left: Input, val right: Input, val result: Wire)

class TwoInputGate(val updateFun: BinaryOp, val left: Input, val right: Input, val result: Wire) : Gate {
    override fun connectWires() {
        if (left is Input.Wire) {
            left.wire.gates.add(this)
        }
        if (right is Input.Wire) {
            right.wire.gates.add(this)
        }
    }

    override fun updateValue() {
        if (left.hasValue() && right.hasValue() && !result.hasValue) {
            result.value = updateFun.fn(left.value(), right.value());
            result.hasValue = true

            for (gate in result.gates) {
                gate.updateValue();
            }
        }        
    }

    override fun toString(): String = "$left $updateFun $right -> $result"
}

data class OneInputGateData(val left: Input, val result: Wire)

class OneInputGate(val updateFun: UnaryOp, val left: Input, val result: Wire) : Gate {
    override fun connectWires() {
        if (left is Input.Wire) {
            left.wire.gates.add(this)
        }
    }

    override fun updateValue() {
        if (left.hasValue() && !result.hasValue) {
            result.value = updateFun.fn(left.value())
            result.hasValue = true

            for (gate in result.gates) {
                gate.updateValue();
            }
        }
    }

    override fun toString(): String = "$updateFun $left -> $result"
}

open class BinaryOp(val fn: (Short, Short) -> Short)

open class UnaryOp(val fn: (Short) -> Short)

object And: BinaryOp({a, b -> (a.toInt() and b.toInt()).toShort()}) {
    override fun toString(): String {
        return "And"
    }
}

object Or: BinaryOp({a, b -> (a.toInt() or b.toInt()).toShort()}) {
    override fun toString(): String {
        return "Or"
    }
}

object RShift: BinaryOp({a, b -> (a.toInt() ushr b.toInt()).toShort()}) {
    override fun toString(): String {
        return "RShift"
    }
}

object LShift: BinaryOp({a, b -> (a.toInt() shl b.toInt()).toShort()}) {
    override fun toString(): String {
        return "LShift"
    }
}

object Not: UnaryOp({a -> a.toInt().inv().toShort()}) {
    override fun toString(): String {
        return "Not"
    }
}
