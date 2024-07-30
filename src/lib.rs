use chrono::prelude::*;

pub struct Cron;
impl Cron {
    pub fn parse_time(cron_string: &str, start_date: Option<chrono::DateTime<Utc>>) -> Result<i64, String>{
        let separated_values: Vec<&str> = cron_string.split(' ').collect();

        if separated_values.len() != 5 && separated_values.len() != 6 {
            return Err(format!("Incorrect amount of parameters in cron string, please review format"));
        }

        let seconds_list: Vec<u32>;
        let minutes_list: Vec<u32>;
        let hours_list: Vec<u32>;
        let days_month_list: Vec<u32>;
        let months_list: Vec<u32>;
        let days_week_list: Vec<u32>;

        let mut array_index = 0;
        if separated_values.len() == 6{
            match Self::parse_crontab_chunk(separated_values[array_index], 0, 60) {
                Ok(n) =>  seconds_list = n,
                Err(e) => return Err(e)
            }
            array_index += 1;
        }
        else{
            seconds_list = [0].to_vec();
        }

        match Self::parse_crontab_chunk(separated_values[array_index], 0, 60) {
            Ok(n) =>  minutes_list = n,
            Err(e) => return Err(e)
        }

        array_index += 1;

        match Self::parse_crontab_chunk(separated_values[array_index], 0, 24) {
            Ok(n) =>  hours_list = n,
            Err(e) => return Err(e)
        }

        array_index += 1;
        match Self::parse_crontab_chunk(separated_values[array_index], 1, 32) {
            Ok(n) =>  days_month_list = n,
            Err(e) => return Err(e)
        }

        array_index += 1;
        match Self::parse_crontab_chunk(separated_values[array_index], 1, 13) {
            Ok(n) =>  months_list = n,
            Err(e) => return Err(e)
        }

        array_index += 1;
        match Self::parse_crontab_chunk(separated_values[array_index], 0, 7) {
            Ok(n) =>  days_week_list = n,
            Err(e) => return Err(e)
        }

        //After that combine and calculate closest execution time delay
        let start_time: DateTime<Utc>;

        match start_date{
            Some(n) => start_time = n,
            None => start_time = chrono::Utc::now()
        }
        
        return Self::get_closest_execution_timer(start_time, months_list, days_month_list, days_week_list, hours_list, minutes_list, seconds_list);
    }

    fn parse_crontab_chunk(chunk: &str, sequence_start: u32, max_value: u32) -> Result<Vec<u32>, String>{
        match chunk.parse::<u32>() {
            Ok(n) => return Self::process_base_case(n, sequence_start, max_value),
            Err(_) => Self::process_list_case(chunk, sequence_start, max_value),
        }
    }

    fn process_base_case(int_val: u32, min_value: u32, max_value: u32) -> Result<Vec<u32>, String> {
        let mut result: Vec<u32> = Vec::new();

        if int_val > max_value.into() || int_val < min_value.into() {
            return Err(format!("Found incorrect value: {int_val}"));
        }

        result.push(int_val);
        return Ok(result);
    }
    
    fn process_star_case(sequence_start: u32, max_value: u32) -> Result<Vec<u32>, String> {
        let mut result: Vec<u32> = Vec::new();

        for i in sequence_start..max_value{
            result.push(i.into());
        }

        return Ok(result);
    }
    
    fn process_divisor_case(chunk: &str, mut sequence_start: u32, mut max_value: u32) -> Result<Vec<u32>, String> {
        let mut result: Vec<u32> = Vec::new();
        let divisor: u32;
        let chunks: Vec<&str> = chunk.split('/').collect();

        if chunks[0].len() == 1 && chunks[0] == "*"
        {
        }
        else
        {
            match chunks[0].parse::<u32>() {
                Ok(n) => {sequence_start = n},
                Err(_) => 
                {
                    if chunks[0].contains('-') {
                        let range_result = Self::process_range_case(chunks[0], max_value);
                        match range_result {
                            Ok(mut n) => {
                                n.sort();
                                sequence_start = *n.first().unwrap();
                                max_value = *n.last().unwrap() + 1;
                            },
                            Err(e) => return Err(e)
                        }
                    }else {
                        return Err(format!("Malformed crontab string"))
                    }
                },
            }
        }

        match chunks[1].parse::<u32>() {
            Ok(n) => {
                if n > max_value || n <= 0{
                    return Err(format!("Malformed crontab string"));
                }else {
                    divisor = n;
                }
            },
            Err(_) => {return Err(format!("Malformed crontab string"));},
        }

        for i in (sequence_start..max_value).step_by(divisor.try_into().unwrap()){
            result.push(i.into());
        }

        return Ok(result);
    }
    
    fn process_range_case( chunk: &str, mut max_value: u32) -> Result<Vec<u32>, String> {
        let mut result: Vec<u32> = Vec::new();
        let chunks: Vec<&str> = chunk.split('-').collect();
        let sequence_start: u32;

        match chunks[0].parse::<u32>() {
            Ok(n) => {sequence_start = n},
            Err(_) => {return Err(format!("Malformed crontab string"));},
        }
        
        match chunks[1].parse::<u32>() {
            Ok(n) => {
                if n > max_value || n < sequence_start {
                    return Err(format!("Malformed crontab string"));
                }else {
                    max_value = n + 1;
                }
            },
            Err(_) => {return Err(format!("Malformed crontab string"));},
        }

        for i in sequence_start..max_value{
                result.push(i.into());
        }

        return Ok(result);
    }
    
    fn process_list_case(chunk: &str, sequence_start: u32, max_value: u32) -> Result<Vec<u32>, String> {
        let mut result: Vec<u32> = Vec::new();
        let chunks: Vec<&str> = chunk.split(',').collect();

        for chunkies in chunks {
            match chunkies.parse::<u32>() {
                Ok(n) => {
                    if n <= max_value && n >= sequence_start {
                            result.push(n.into());
                    }else {
                        return Err(format!("Malformed crontab string"));
                    }
                },
                Err(_) => 
                {
                    if chunkies.len() == 1 && chunkies == "*" {
                        return Self::process_star_case(sequence_start, max_value);
                    }else{
                        if chunkies.contains('/') {
                            let div_results = Self::process_divisor_case(chunkies, sequence_start, max_value);

                            match div_results{
                                Ok(n) => for div_res in n { result.push(div_res) },
                                Err(e) => return Err(e)
                            }
                        }else if chunkies.contains('-'){
                            let range_results = Self::process_range_case(chunkies, max_value);

                            match range_results{
                                Ok(n) => for div_res in n { result.push(div_res) },
                                Err(e) => return Err(e)
                            }
                        }else {
                            return Err("Malformed crontab string".to_owned())
                        }
                    }
                },
            }
        }

        return Ok(result);
    }
    
    fn get_closest_execution_timer(mut now: DateTime<Utc>, months_list: Vec<u32>, days_month_list: Vec<u32>, days_week_list: Vec<u32>, hours_list: Vec<u32>, minutes_list: Vec<u32>, seconds_list: Vec<u32>) -> Result<i64, String> {
        if months_list.contains(&now.month())
        {
            if days_month_list.contains(&now.day()) && days_week_list.contains(&now.weekday().num_days_from_sunday())
            {
                for hour in &hours_list
                {
                    if now.hour() < *hour
                    {
                        now = chrono::Utc.with_ymd_and_hms(now.year(), now.month(), now.day(), *hour, minutes_list[0], 0).unwrap(); // I have faith in myself, this won't fail
                        let result = now - chrono::Utc::now();
                        return Ok(result.num_milliseconds());
                    }
                    else if now.hour() == *hour
                    {
                        for minute in &minutes_list
                        {
                            if now.minute() < *minute
                            {
                                now = chrono::Utc.with_ymd_and_hms(now.year(), now.month(), now.day(), now.hour(), *minute, seconds_list[0]).unwrap(); // I have faith in myself, this won't fail
                                let result = now - chrono::Utc::now();
                                return Ok(result.num_milliseconds());
                            }
                            else if now.minute() == *minute{
                                for second in &seconds_list
                                {
                                    if now.second() < *second
                                    {
                                        now = chrono::Utc.with_ymd_and_hms(now.year(), now.month(), now.day(), now.hour(), *minute, *second).unwrap(); // I have faith in myself, this won't fail
                                        let result = now - chrono::Utc::now();
                                        return Ok(result.num_milliseconds());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            now = chrono::Utc.with_ymd_and_hms(now.year(), now.month(), now.day(),0,0,0).unwrap();
            return Self::get_closest_execution_timer(now.checked_add_days(chrono::Days::new(1)).unwrap(), months_list, days_month_list, days_week_list, hours_list, minutes_list, seconds_list);
        }
        now = chrono::Utc.with_ymd_and_hms(now.year(), now.month(), 1, 0, 0, 0).unwrap();
        return Self::get_closest_execution_timer(now.checked_add_months(chrono::Months::new(1)).unwrap(), months_list, days_month_list, days_week_list, hours_list, minutes_list, seconds_list);
    }
}