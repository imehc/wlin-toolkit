/// 数字大写转换 
pub fn digit_uppercase(value: f64) -> Result<String, &'static str> {
    // 1. 把上限改成">= 1000 亿"就报错
    if !value.is_finite() || value < 0.0 || value >= 100_000_000_000.0 {
        return Err("Invalid number");
    }

    let int_part = value.trunc() as u128;
    let dec_part = ((value.fract() * 100.0).round() as u8).min(99);

    const NUM: [&str; 10] = ["零", "壹", "贰", "叁", "肆", "伍", "陆", "柒", "捌", "玖"];
    const UNIT: [&str; 4] = ["", "拾", "佰", "仟"];
    const SEC: [&str; 3] = ["", "万", "亿"];

    // ---------- 整数部分 ----------
    let mut int_str = String::new();
    let mut n = int_part;
    let mut sec_idx = 0usize;

    // 从低位到高位，每 4 位一组
    while n > 0 || int_str.is_empty() {
        let mut seg = String::new();
        let grp = n % 10_000;
        n /= 10_000;

        let mut zero = false;
        let mut any = false;
        for pos in (0..4).rev() {
            let d = (grp / 10_u128.pow(pos)) % 10;
            if d != 0 {
                if zero {
                    seg.push_str("零");
                    zero = false;
                }
                seg.push_str(NUM[d as usize]);
                seg.push_str(UNIT[pos as usize]);
                any = true;
            } else {
                zero = true;
            }
        }

        if any {
            if sec_idx > 0 {
                seg.push_str(SEC[sec_idx]);
            }
            int_str = format!("{}{}", seg, int_str);
        } else if sec_idx > 0 && !int_str.is_empty() {
            // 中间出现全 0 的万级/亿级，补零
            int_str = format!("零{}", int_str);
        }

        sec_idx += 1;
        if n == 0 {
            break;
        }
    }

    // ---------- 小数部分 ----------
    let dec_str = match dec_part {
        0 => "整".to_string(),
        _ => {
            let mut s = String::new();
            let jiao = dec_part / 10;
            let fen = dec_part % 10;
            if jiao > 0 {
                s.push_str(NUM[jiao as usize]);
                s.push_str("角");
            }
            if fen > 0 {
                s.push_str(NUM[fen as usize]);
                s.push_str("分");
            }
            s
        }
    };

    // ---------- 拼接 ----------
    let mut res = if int_part > 0 {
        format!("{}元{}", int_str.trim_start_matches("零"), dec_str)
    } else {
        format!("零元{}", dec_str)
    };

    // 去掉多余"零零"
    res = res.replace("零零", "零");
    Ok(res.trim_end_matches('零').to_string())
}