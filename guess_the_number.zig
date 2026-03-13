const std = @import("std");

pub fn main() !void {
    const stdin = std.io.getStdIn().reader();
    const secret_number = try random(1, 100);
    var buf: [1024]u8 = undefined;

    while (true) {
        try std.debug.print("guess the number (1 - 100): ", .{});

        if (try stdin.readUntilDelimiterOrEof(&buf, '\n')) |line| {
            // parse base 10 integer
            if (std.fmt.parseInt(u8, line, 10)) |guess| {
                std.debug.print("you guessed {d}\n", .{guess});

                switch (std.math.order(guess, secret_number)) {
                    .lt => std.debug.print("too small\n", .{}),
                    .gt => std.debug.print("too large\n", .{}),
                    .eq => {
                        std.debug.print("you win!\n", .{});
                        break;
                    },
                }
            } else |err| {
                std.debug.print("please enter a valid integer: {}\n", .{err});
            }
        } else {
            std.debug.print("failed to read line", .{});
        }
    }
}

fn random(min: u8, max: u8) !u8 {
    var seed: u64 = undefined;
    try std.posix.getrandom(std.mem.asBytes(&seed));

    var prng: std.Random.DefaultPrng = .init(seed);
    const rand = prng.random();

    return rand.intRangeAtMost(u8, min, max);
}
