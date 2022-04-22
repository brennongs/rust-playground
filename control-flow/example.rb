def fizzbuz
  num = 0
  while num <= 100 do
    num++
    puts num unless num % 5 == 0 || num % 3 == 0
    puts 'fizz' if num % 3 == 0
    puts 'buzz' if num % 5 == 0

    puts '\n'
  end
end