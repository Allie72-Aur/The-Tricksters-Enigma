use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    let mut rng = rand::thread_rng();

    // --- Opening Dialogue: A few options to make the start unpredictable ---
    let openings = [
        "Aha! A curious soul has come to play. I've tangled a number in a web of whispers from 1 to 100. Will you unravel my little secret?",
        "Greetings, seeker! A number sleeps somewhere in the realm of 1 to 100. I dare you to wake it with your mind, if you can!",
        "Well, well. Fancy a game of wits? My number is a mischievous ghost, flitting between 1 and 100. Let's see if you can pin it down.",
        "Welcome! I’ve plucked a number from the stars, between 1 and 100, and hidden it just for you to find. Think of a number, any number... but is it the right one?",
        "Greetings, brave guesser! I'm holding a secret number, a tiny mystery between 1 and 100. Dare to uncover it?",
        "Ah, a new challenger appears! I've conjured a number, a mischievous little sprite dwelling somewhere from 1 to 100. Can you coax it out?",
        "Welcome, curious mind! I've tucked away a number, a whisper from 1 to 100. Think you can hear its call?",
        "Aha! A challenger approaches! I've hidden a number, a playful spirit, somewhere between 1 and 100. Can you summon it with your intuition?",
        "Ah, a brave soul! I've woven a number into the fabric of the universe, somewhere between 1 and 100. Can you see it with your mind's eye?",
        "Ah, a curious mind has arrived! I've hidden a number, a playful enigma, somewhere between 1 and 100. Can you unravel its mystery?",
        "Ah, a seeker of secrets! I've tucked away a number, a playful riddle, somewhere between 1 and 100. Can you solve it with your intuition?",
        "Fancy a game? I've got a number playing hide-and-seek from 1 to 100. Let's see if your intuition is sharp enough to find it!",
        "A number hides, a ghost in the machine, and you've come to hunt it! How deliciously daring of you.",
        "The great game is afoot! I've bound a number with a riddle. Can you break the spell?",
        "I've chosen a number, a mischievous little imp, between 1 and 100. It's waiting to be found... or not!",
        "Welcome to my little game of chance and cunning! My secret number awaits its fate.",
    ];

    println!("\n====================================================");
    println!("{}", openings[rng.gen_range(0..openings.len())]);
    println!("====================================================\n");

    let secret_number: u8 = rng.gen_range(1..=100);

    // This debug message is only shown when the program is compiled in debug mode.
    #[cfg(debug_assertions)]
    println!("(Psst! The secret number is {secret_number}. Don't tell anyone I told you!)");

    loop {
        // Prompt the player for a number
        let prompts = [
            "Your wisdom, please. What's your next magnificent guess? ",
            "Speak your guess into the ether... what number do you see? ",
            "Whisper your number to the winds... what do they tell you? ",
            "What number dances in your mind? Speak it aloud! ",
            "Let your intuition guide you... what number emerges from the shadows? ",
            "What number do you conjure from the depths of your imagination? ",
            "What is your next bold declaration?",
            "Unveil your next theory, if you dare...",
            "Cast your spell of a number!",
        ];
        print!("{}", prompts[rng.gen_range(0..prompts.len())]);
        // flush stdout to ensure prompt appears before input
        let _ = io::stdout().flush();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Reading a line should not Fail");

        // --- Invalid Input: Now with more trickster-like confusion ---
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                let invalid_inputs = [
                    "Is that a secret spell? My ears hear only gibberish. Try again with a number.",
                    "Ah, I see you're using the language of the lost. Alas, I only understand numbers! Give it another try.",
                    "My, my! That's not a number. Are you trying to trick me? Be a good sport and give me a number.",
                    "A fascinating gesture, but it doesn't quite translate. Please, a simple number will do the trick.",
                    "Ah, a curious gesture, but alas, it does not translate into the language of numbers. Please, a simple digit will suffice.",
                    "A fascinating display, but it seems my ears are attuned to a different frequency. A number, if you please!",
                    "Ah, a fine poem! But my game needs prose, not poetry. Or rather, digits, not words.",
                    "That's a lovely sound, but it doesn't quite resonate with my game's frequency. A number, please!",
                    "Hmm, are you trying to speak to the wind? It doesn't listen to that either. Try a number.",
                    "Fascinating! But my game has no place for such... curiosities. A number, if you please.",
                ];
                println!("{}", invalid_inputs[rng.gen_range(0..invalid_inputs.len())]);
                continue;
            }
        };

        #[cfg(debug_assertions)]
        println!("You ventured: {guess}.");

        // Calculate the difference to determine how close the guess is.
        // We cast to i32 to avoid underflow with the absolute difference.
        let difference: u8 = (guess as i32 - secret_number as i32).abs() as u8;

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                // --- 'Too Small' Responses: Now with misleading clues and metaphors ---
                if difference <= 5 {
                    let hot_responses = [
                        "So close! You can almost feel the air shift... but my number is taller than that. A little taller.",
                        "A spark of warmth! You've found the right neighborhood, but you're looking at the wrong house. It's just past it.",
                        "Almost! The shadow of my number touches your guess. Look up!",
                        "You're dancing on the edge of a cliff! My number is just one step further out. Be careful not to fall!",
                        "The scent of truth is strong! But you're sniffing at the flower, not the root. Dig a little deeper!",
                        "A breath away! But remember, a breath can be a gust of wind. It’s slightly more than that.",
                    ];
                    println!("{}", hot_responses[rng.gen_range(0..hot_responses.len())]);
                } else if difference <= 20 {
                    let warm_responses = [
                        "Warmer, but the sun is setting! Your guess is a fine one, but a different kind of fine. It's somewhere else.",
                        "You're chasing the wrong echo! That number is singing, but not your tune. It's a different note entirely.",
                        "A good try! But my number lives higher on the mountain, where the air is thin.",
                        "You're sailing the seas, but my number is on a different continent. Keep the rudder pointed 'more'!",
                        "A noble effort, but you're only halfway up the staircase. My number is waiting on a higher landing.",
                        "That's a fine guess, but it's a seed that needs to grow into a much larger tree.",
                    ];
                    println!("{}", warm_responses[rng.gen_range(0..warm_responses.len())]);
                } else {
                    let cold_responses = [
                        "My, my! Your guess is a whisper, but my number is a shout. You'll need to use your lungs a bit more!",
                        "Your guess is a tiny pebble, but my number is a giant boulder! You'll need a much bigger rock to find it.",
                        "Oh, my! You've found a different game entirely! My number is much larger than that.",
                        "My number is a roaring fire, and you've thrown a single match. It needs a much bigger flame!",
                        "You've wandered into the desert. The oasis you seek is far, far ahead.",
                    ];
                    println!("{}", cold_responses[rng.gen_range(0..cold_responses.len())]);
                }
            }
            Ordering::Greater => {
                // --- 'Too Big' Responses: With more chaotic, indirect hints ---
                if difference <= 5 {
                    let hot_responses = [
                        "Whoa! That's a bit too much fire. You're so close, but you've flown right over the candle. Backtrack!",
                        "The scent is strong! But your guess is a bit too much. It needs to be less... full.",
                        "You're practically there! But you've built your tower too high. My number is on a lower floor.",
                        "A bit too much! You've overfed the beast. Let it go hungry a little and it will be perfect.",
                        "You've nearly got it! But you're squeezing too tight. Loosen your grip on that number just a bit.",
                        "The answer is a whisper, and your guess is a yell. Bring the volume down, just a tad.",
                    ];
                    println!("{}", hot_responses[rng.gen_range(0..hot_responses.len())]);
                } else if difference <= 20 {
                    let warm_responses = [
                        "A good try, but you've sailed past the harbor. The treasure is on a different shore!",
                        "You've got the wrong constellation! Your number is close, but mine is a star on a different part of the map.",
                        "Getting warmer! But that's a different mountain. The one you seek is a little smaller.",
                        "You're in the right forest, but you're chopping down the wrong tree. The one you seek is a bit smaller.",
                        "My number is not that high. It's a humble hill, not a towering mountain.",
                        "A fine attempt, but your star shines a bit too brightly. My number is a faint twinkle.",
                    ];
                    println!("{}", warm_responses[rng.gen_range(0..warm_responses.len())]);
                } else {
                    let cold_responses = [
                        "You've ventured into the clouds! My number is a root in the earth. Try a completely different direction.",
                        "Oh, my! Your number is a meteor, but mine is a star. One is flying, the other is standing still.",
                        "Your guess is a thunderclap, but my number is a gentle patter. Less... volume, perhaps?",
                        "My number is a quiet mouse, and your guess is a roaring lion! Less sound and fury, please!",
                        "You've guessed the destination of a different journey! My number is much, much earlier on the map.",
                        "Whoa there, you've gone right over the top! You've sailed past the end of the world. Come back!",
                    ];
                    println!("{}", cold_responses[rng.gen_range(0..cold_responses.len())]);
                }
            }
            Ordering::Equal => {
                // --- Winning Responses: The trickster is surprised and impressed ---
                let winning_responses = [
                    "You've done it! I can't believe it! The secret is out, and I thought I had it so well-hidden!",
                    "Astounding! You've plucked the very number from my thoughts! My magic is... a bit rusty, it seems. Congratulations!",
                    "Incredible! A perfect match. The stars have aligned for you today, and I am, for once, speechless.",
                    "You win! I must admit, I'm a little shocked. My game has been bested by a mere mortal!",
                    "A perfect match! I suppose you've proven your prowess. The game bows before you, though I'm not too happy about it.",
                    "Bravo! You've outsmarted me this time. I thought I had you tangled in my web of misdirection, but you slipped right through!",
                    "Eureka! You've done it! The secret is unveiled! A grand cheer for your magnificent mind!",
                    "Astounding! You've plucked the very number from my thoughts! A truly masterful victory!",
                    "Incredible! You are a true number whisperer! Congratulations, champion!",
                    "Huzzah! The stars align, and your guess hits the bullseye! Victory is yours!",
                    "A perfect match! You've proven your prowess. The game bows before you, victor!",
                    "Well done! You've unraveled my little mystery. I must say, I'm impressed. Not many can outwit me!",
                    "Blast! The spell is broken! You've cracked my code. A magnificent win, you scoundrel!",
                    "I'm not saying you cheated, but that was far too quick. You must have a little bit of my magic in you! Fine, you win.",
                    "My game has been solved! I'm so disappointed... for me, that is. Well played.",
                ];
                println!(
                    "\n{}",
                    winning_responses[rng.gen_range(0..winning_responses.len())]
                );
                println!("====================================================\n");
                break;
            }
        }
    }
}
