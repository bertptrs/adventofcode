{
    solve = input_file:
        let
            data = builtins.readFile input_file;
            lines = builtins.filter (s: s != "" && builtins.isString s) (builtins.split "\n" data);

            # Basic maths because the standard library doesn't have it
            mod = a: b: a - (a / b) * b;
            abs = n: if n < 0 then -n else n;

            recurse = list: position: score1: score2:
                let
                    first = builtins.head list;
                    # This is not guaranteed to work but it's good enough
                    num_part = builtins.substring 1 999 first;
                    num = builtins.fromJSON num_part;
                    delta = mod (if (builtins.substring 0 1 first) == "R" then num else -num) 100;
                    next = mod (position + delta + 100) 100;
                    
                    score = if next == 0 then 1 else 0;
                    circles = num / 100;

                    passed = if position == 0 then
                        0
                    else
                        if delta < 0 && delta + position <= 0 || delta > 0 && delta + position >= 100 then
                            1
                        else
                            0;
                in
                    if list == [] then
                        [score1 score2]
                    else
                        recurse (builtins.tail list) next (score + score1) (score2 + passed + circles);
        in
            recurse lines 50 0 0;
}
