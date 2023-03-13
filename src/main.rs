// CSV-powered Rustlang Analyzer for Ballots
// use std::collections::{HashMap, HashSet};
use std::collections::{HashMap};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use structopt::StructOpt;

#[derive(Clone)]
struct Ballot {
    uuid: String,
    choices: Vec<String>,
}

#[derive(StructOpt)]
#[structopt(about = "This app will dynamically load a CSV, compute Ranked Choice Voting, and provide simple stats.")]
struct CLIArguments {
    #[structopt(short = "f", long = "file", help = "The input file to read from")]
    input_file: Option<String>,

    #[structopt(short = "v", long = "verbose", help = "Print verbose output")]
    verbose: bool,

    #[structopt(long = "help", help = "Prints help information")]
    help: bool,
}

//ChatGPT:
fn sort_hashmap_by_key(map: &mut HashMap<usize, i32>) -> Vec<(usize, i32)> {
    let mut sorted_map: Vec<(usize, i32)> = map.iter().map(|(&k, &v)| (k, v)).collect();
    sorted_map.sort_by_key(|&x| x.0);
    sorted_map
}

//ChatGPT
fn sort_hashmap_by_string_key(map: &mut HashMap<String, i32>) -> Vec<(String, i32)> {

    let mut sorted_map_borda = Vec::new();
    for (key, value) in map {
        sorted_map_borda.push((key.clone(), *value));
    }
    sorted_map_borda.sort_by(|a, b| b.1.cmp(&a.1));
    sorted_map_borda

    // let mut sorted_map: Vec<(String, i32)> = map.iter().map(|(k, v)| (k.clone(), *v)).collect();
    // sorted_map.sort_by_key(|x| x.0.clone());
    // sorted_map
}

//ChatGPT: return the scoring of each candidate -- https://en.wikipedia.org/wiki/Borda_count#Ballot
fn borda_count_scores(choices: &Vec<String>, votes: &Vec<Ballot>) -> Vec<i32> {
    let num_choices = choices.len();
    let mut scores = vec![0; num_choices];

    for vote in votes {
        for (i, candidate) in vote.choices.iter().enumerate() {
            if let Some(index) = vote.choices.iter().position(|c| c == candidate) {
                scores[index] += (num_choices - i - 1) as i32;
            }
        }
    }

    scores
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("CSV-powered Rustlang Analyzer for Ballots (CRAB)");
    println!("================================================");

    let args = CLIArguments::from_args();

    if args.help {
        CLIArguments::clap().print_help()?;
        println!();
        println!();
        println!("[!WARNING!] The first column is skipped -- treated as ID [!WARNING!]");
        println!();
        return Ok(());
    }

    let file = File::open(args.input_file.unwrap_or_else(|| "votes.csv".to_string()))?;
    let reader = BufReader::new(file);
    let mut ballots = Vec::new();
    let mut total_choices_in_all_ballots = 0;
    let mut discovered_choices = Vec::new();

    let top_to_bottom_list_limit = 0; //Top 20

    for line in reader.lines() {
        let line = line?;
        let mut read_in_ballot = Ballot {
            uuid: String::new(),
            choices: Vec::new(),
        };

        //Read in the UUID:
        let mut read_uuid = String::new();
        for c in line.chars() {
            if c == ',' {
                break;
            }
            read_uuid.push(c);
        }
        read_in_ballot.uuid = read_uuid;

        //Read in the choices && store unique choices in `discovered_choices`
        let mut read_choice = String::new();
        for c in line.chars().skip(read_in_ballot.uuid.len() + 1) {
            if c == ',' {
                if !read_choice.is_empty() {
                    read_in_ballot.choices.push(read_choice.clone());
                    total_choices_in_all_ballots += 1;

                    if !discovered_choices.contains(&read_choice) {
                        discovered_choices.push(read_choice.clone());
                    }
                }
                read_choice = String::new();

            } else {
                read_choice.push(c);
            }

            if c == line.chars().last().unwrap() && !read_choice.is_empty() {
                read_in_ballot.choices.push(read_choice.clone());
                total_choices_in_all_ballots += 1;

                if !discovered_choices.contains(&read_choice) {
                    discovered_choices.push(read_choice.clone());
                }

            }

        }

        ballots.push(read_in_ballot);
    }

    discovered_choices.retain(|x| !x.is_empty());
    discovered_choices.sort();

    println!("[CRAB] Total Ballots: {}", ballots.len());
    println!("[CRAB] Total Choices: {}", discovered_choices.len());
    println!("[CRAB] Discovered Choices: {:?}", discovered_choices);

    let mut ballots_rcv_analysis = ballots.clone();

    let mut top_to_bottom_list = Vec::new();
    let mut top_to_bottom_list_counter=0;

    loop {
        let mut rcv_downselect_loop_counter = 0;
        loop {
            if args.verbose {
                println!("[CRAB] [VERBOSE] [{}] RCV Loop Iteration: {}", top_to_bottom_list_counter, rcv_downselect_loop_counter);
            }

            // let mut counter = 0;
            let mut vote_counts = HashMap::new();
            for ballot in &ballots_rcv_analysis {
                //check if ballot.choices[0] is empty, if so, skip
                if ballot.choices.is_empty() {
                    continue;
                }

                // println!("[CRAB] [VERBOSE] [{}] Ballot #{}", top_to_bottom_list_counter, counter);
                // println!("[CRAB] [VERBOSE] [{}] UUID: {}", top_to_bottom_list_counter, ballot.uuid);
                // println!("[CRAB] [VERBOSE] [{}] Choice: {}", top_to_bottom_list_counter, ballot.choices[0]);

                let count = vote_counts.entry(ballot.choices[0].clone()).or_insert(0);
                *count += 1;
            }

            if args.verbose {
                println!("[CRAB] [VERBOSE] [{}] Vote Counts: {:?}", top_to_bottom_list_counter, vote_counts);
            }

            let total_ballot_votes = vote_counts.values().sum::<i32>();
            let mut largest_ballot_sum = 0;
            let mut largest_ballot_choice = String::new();

            for (key, value) in &vote_counts {
                if value > &largest_ballot_sum {
                    largest_ballot_sum = *value;
                    largest_ballot_choice = key.clone();
                }
            }
            let choice_percent = largest_ballot_sum as f32 / total_ballot_votes as f32 * 100.0;

            if args.verbose {
                println!("[CRAB] [VERBOSE] [{}] Largest Choice: {} - @{} ({}%)", top_to_bottom_list_counter, largest_ballot_choice, largest_ballot_sum, choice_percent);
            }

            if largest_ballot_sum > total_ballot_votes / 2 {
                if args.verbose {
                    println!("[CRAB] Winner ({}): {}", top_to_bottom_list_counter+1, largest_ballot_choice);
                }
                top_to_bottom_list.push(largest_ballot_choice.clone());
                break;
            }

            let mut smallest_ballot_sum = 0;
            let mut smallest_ballot_choice = String::new();
            //Determine smallest ballot sum
            for (key, value) in &vote_counts {

                if value < &smallest_ballot_sum || smallest_ballot_sum == 0{
                    smallest_ballot_sum = *value;
                    smallest_ballot_choice = key.clone();
                }
            }

            if args.verbose {
                println!("[CRAB] [VERBOSE] [{}] Smallest Choice {} - @{}", top_to_bottom_list_counter, smallest_ballot_choice, smallest_ballot_sum);
            }

            // loop over vote_counts, if the value is equal to smallest_ballot_sum, remove ballot.choices[0]
            for (key, value) in &vote_counts {
                let eval_choice = String::from(key);
                if value == &smallest_ballot_sum {
                    for ballot in &mut ballots_rcv_analysis {
                        //check if ballot.choices is empty
                        if ballot.choices.is_empty() {
                            continue;
                        }
                        if ballot.choices[0] == eval_choice {
                            ballot.choices.remove(0);
                        }
                    }
                }
            } 

            rcv_downselect_loop_counter += 1;

            if args.verbose {
                println!("[CRAB] [VERBOSE] [{}] ",top_to_bottom_list_counter);
            }

        }
        //latest winner pushed to top_to_bottom_list; current ballots in ballots_rcv_analysis 

        top_to_bottom_list_counter += 1;
        if top_to_bottom_list_counter >= top_to_bottom_list_limit && top_to_bottom_list_limit != 0{
            break;
        }

        //println!("[CRAB] [DEBUG] top_to_bottom_list_counter {} vs. discovered_choices.len() {}", top_to_bottom_list_counter, discovered_choices.len());
        if top_to_bottom_list_counter >= discovered_choices.len()+1 {
            break;
        }

        let mut temp_ballots_rcv_analysis = ballots_rcv_analysis.clone();
        // let mut temp_ballot = ballots_rcv_analysis[0].clone();
        temp_ballots_rcv_analysis.clear();
        ballots_rcv_analysis.clear();

        for ballot in &ballots {
            let mut temp_ballot = ballot.clone();
            
            for winner in &top_to_bottom_list {
                if temp_ballot.choices.contains(winner) {
                    temp_ballot.choices.remove(temp_ballot.choices.iter().position(|x| x == winner).unwrap());
                }
            }
            //push into ballots_rcv_analysis
            temp_ballots_rcv_analysis.push(temp_ballot.clone());
        }

        for temp_ballot in &temp_ballots_rcv_analysis {
            if !temp_ballot.choices.is_empty() {
                ballots_rcv_analysis.push(temp_ballot.clone());
            }
        }
        // break if ballots_rcv_analysis is empty
        if ballots_rcv_analysis.is_empty() {
            break;
        }

    }
    //END top_to_bottom_list LIMITED loop
    println!();
    println!("============ Winner ============");
    println!("[CRAB] Winner List (1st to nth): {:?}", top_to_bottom_list);


    // ---------------------------------------------------------------------------
    // //EXPERIMENTAL -- Vote counts, Stats, and Bordas---

    let boarda_scores = borda_count_scores(&discovered_choices, &ballots);

    if args.verbose {
        //generate a "Statsitics Banner" for println
        println!();
        println!("============ Statsitics ============");
        println!("[CRAB] [DEBUG] Total Ballots: {}", ballots.len());
        println!("[CRAB] [DEBUG] Choices: {}", discovered_choices.len());
        println!("[CRAB] [DEBUG] Total Choices in Ballots: {}", total_choices_in_all_ballots);

        println!();
        println!("------------ Borda Scoring ------------");

        //Print Length of discovered_choices
        println!("[CRAB] [BORDA] Length of discovered_choices: {}", discovered_choices.len());
        println!("[CRAB] [BORDA] Ranking Score Value: {:?}", boarda_scores);

    }

    // create new hashmap to store vote counts
    let mut choices_vote_counts = HashMap::new();
    let mut choices_total_borda_count = HashMap::new();

    for choice in &discovered_choices {

        let hard_coded_value = choice.clone();

        //Vote Count
        //==========
        let mut position_count = HashMap::new();
        for ballot in &ballots {
            for (i, choice) in ballot.choices.iter().enumerate() {
                if *choice == hard_coded_value {
                    let count = position_count.entry(i).or_insert(0);
                    *count += 1;
                }
            }
        }

        //check positon_count elements vs. discovered_choices elements -- add 0's if needed
        for i in 0..discovered_choices.len() {
            if !position_count.contains_key(&i) {
                position_count.insert(i, 0);
            }
        }

        // Sort count
        //-----------
        let sorted_map = sort_hashmap_by_key(&mut position_count);
        choices_vote_counts.insert(hard_coded_value.clone(), sorted_map.clone());

        let mut position_count_values = Vec::new();
        for (_key, value) in sorted_map {
            position_count_values.push(value);
        }


        //Borda Count
        let mut total_borda_value = 0;
        if position_count_values.len() == boarda_scores.len(){
            let calc_len = position_count_values.len();

            for i in 0..(calc_len) {
                total_borda_value += position_count_values[i] * boarda_scores[i];

            }
        }
        choices_total_borda_count.insert(hard_coded_value.clone(), total_borda_value);

    }

    if args.verbose {
        println!();
        println!("------------ Vote Count ------------");            
        let mut total_running_count = 0;
        for choice in &discovered_choices {
            let hard_coded_value = choice.clone();
            let sorted_map = choices_vote_counts.get(&hard_coded_value).unwrap();

            let mut position_count_sorted = String::new();
            let mut running_count = 0;
            for (_key, value) in sorted_map {
                position_count_sorted.push_str(&format!("{}, ", value));
                running_count += value;
            }
            total_running_count += running_count;
            position_count_sorted.pop();
            position_count_sorted.pop();
    
            println!("[CRAB] [VOTES] [{}] <{}> |{}|", hard_coded_value, position_count_sorted, running_count);
        }
        println!("[CRAB] [VOTES] Total Vote Count: ||{}||", total_running_count);

        println!();
        println!("------------ Borda Scoring ------------");    
        for choice in &discovered_choices {
            let hard_coded_value = choice.clone();        
            let total_borda_value = choices_total_borda_count.get(&hard_coded_value).unwrap();
            println!("[CRAB] [BORDA] Total Borda Value: [{}] {}", hard_coded_value, total_borda_value);            
        }


    }

    println!();
    println!("========= Sorted Borda Scoring =========");    
    let sorted_map_borda = sort_hashmap_by_string_key(&mut choices_total_borda_count);
    // println!("[CRAB] [BORDA] Ranking Score Value: {:?}", sorted_map_borda);
    for (key, value) in sorted_map_borda {
        println!("[CRAB] [BORDA] Total Borda Value: [{}] {}", key, value);            
    }



    Ok(())
}
