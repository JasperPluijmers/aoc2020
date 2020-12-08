from enum import Enum
import re


class Operation(Enum):
    ACC = 'acc'
    JMP = 'jmp'
    NOP = 'nop'


class State(Enum):
    RUNNING = 0
    FINISHED = 1
    LOOPED = 2


class Console():
    def __init__(self, program):
        self.accumulator = 0
        self.pointer = 0
        self.previous_pointers = set()
        self.program = program
        self.operation_mapping = {Operation.ACC: lambda value: self.execute_acc(value),
                                  Operation.JMP: lambda value: self.execute_jmp(value),
                                  Operation.NOP: lambda value: self.execute_nop()}

    def execute_step(self):
        if self.pointer in self.previous_pointers:
            return State.LOOPED, self.accumulator
        self.previous_pointers.add(self.pointer)
        try:
            operation, value = self.program[self.pointer]
        except IndexError:
            return State.FINISHED, self.accumulator

        self.operation_mapping[operation](value)
        return State.RUNNING, self.accumulator

    def execute_acc(self, value: int):
        self.accumulator += value
        self.pointer += 1

    def execute_jmp(self, value):
        self.pointer += value

    def execute_nop(self):
        self.pointer += 1


def run_program(program):
    console = Console(program)
    while True:
        state, value = console.execute_step()
        if state != State.RUNNING:
            break
    return state, value


with open('input.txt') as f:
    inputs = [re.findall(r'([a-z]+) ([+-]\d+)', line)[0] for line in f.readlines()]
program = [(Operation(operation), int(value)) for operation, value in inputs]

print(run_program(program))
for index, (operation, value) in enumerate(program):
    if operation == Operation.JMP:
        program[index] = (Operation.NOP, value)
        state, final_value = run_program(program)
        if state == State.FINISHED:
            print(final_value)
        program[index] = (Operation.JMP, value)
    if operation == Operation.NOP:
        program[index] = (Operation.JMP, value)
        state, final_value = run_program(program)
        if state == State.FINISHED:
            print(final_value)
        program[index] = (Operation.NOP, value)


