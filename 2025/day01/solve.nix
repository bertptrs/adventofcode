{
    solve = input_file:
        let
            data = builtins.readFile input_file;
            lines = builtins.filter (s: s != "" && builtins.isString s) (builtins.split "\n" data);

            mod = a: b: a - (a / b) * b;

            recurse = list: position:
                let
                    first = builtins.head list;
                    # This is not guaranteed to work but it does
                    num_part = builtins.substring 1 999 first;
                    num = builtins.fromJSON num_part;
                    next = if (builtins.substring 0 1 first) == "R"
                        then
                            position + num
                        else
                            position - num;

                    new_position = mod ((mod next 100) + 100) 100; 
                    
                    score = if new_position == 0 then 1 else 0;
                in
                    if list == [] then
                        0
                    else
                        score + (recurse (builtins.tail list) new_position);
        in
            recurse lines 50;
}
