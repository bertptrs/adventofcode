function [cycles,repeat] = solution (division)
  division = vec(division);
  states = zeros(0, size(division)(1));
  
  while ! ismember(transpose(division), states, "rows")
    states = [states; transpose(division)];
    
    [maxval, maxindex] = max(division);
    division(maxindex) = 0;
    
    for i = 1:maxval
      targetIndex = mod(maxindex + i - 1, size(division)(1)) + 1;
      division(targetIndex) += 1;
    end
    
  end
  
  cycles = size(states)(1);
  repeat = division;
endfunction
