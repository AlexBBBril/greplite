fn main() {
    let ctx_line = 2;
    let needle = "oo";
    let haystack = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

    let mut tags: Vec<usize> = vec![]; // В tags хранятся номера соответствующих поиску строк
    let mut ctx: Vec<Vec<(usize, String)>> = vec![]; // ctx содержит вектор для каждого соответствия, в котором хранятся контекстные строки

    // Последовательный перебор строк с записью номеров тех из них, где найдены соответствия
    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);

            // Vec::with_capacity(n) резервирует место для n элементов.
            let v = Vec::with_capacity(2 * ctx_line + 1);
            ctx.push(v);
        }
    }

    if tags.is_empty() {
        return;
    }

    // Проверка в каждой строке для каждой метки, достаточно ли эта строка близка к
    // найденному соответствию. Если да, то добавление этой строки к соответствующему
    // Vec<T> в ctx.
    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {

            // saturating_sub () - это вычитание, возвращающее О при исчезновении значащих
            // разрядов целочисленного значения, что позволяет избежать сбоя программы
            //     (процессорам не нравится попытка отправлять значение usize, ставшее ниже нуля)
            let lower_bound = tag.saturating_sub(ctx_line);
            let upper_bound = tag + ctx_line;

            if (i >= lower_bound) && (i <= upper_bound) {

                // Копирование строк в новое значение типа String, локально сохраняемое для каждого
                // найденного соответствия
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {

        // ref line информирует компилятор, что это значение нужно не перемещать, а заимствовать.
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
