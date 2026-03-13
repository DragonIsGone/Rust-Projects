const std = @import("std");

pub fn main() !void {
    const stdin = std.io.getStdIn().reader();
    const stdout = std.io.getStdOut();

    try stdout.writeAll("enter temp: ");

    // fixed size, so its on the stack
    var buf: [1024]u8 = undefined;

    // reads from stdin until delimeter encountered
    // i.e. enter is pressed
    //
    // |line| is a payload capture that unwraps the returned optional,
    // as the user could have entered no data (e.g. EOF)
    if (try stdin.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        // {s} specifies line as a string
        // std.debug.print("user entered: {s}\n", .{line});

        if (std.fmt.parseFloat(f32, line)) |value| {
            // {d} specifies decimal
            // std.debug.print("float is: {d}\n", .{value});

            convert(value);
        } else |err| {
            std.debug.print("please enter a valid float: {}\n", .{err});
        }
    } else {
        std.debug.print("please enter a valid float\n", .{});
    }
}

fn convert(input: f32) void {
    const result = input * 1.8 + 32.0;
    std.debug.print("{d}°F\n", .{result});
}
