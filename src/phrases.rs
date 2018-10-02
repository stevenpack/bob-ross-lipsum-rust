
use std::vec::Vec;
// use rand::RngCore;
use rand::distributions::{Range, Distribution};
//use prng_seeded::{Prng,SeedType};
use rand::rngs::SmallRng;
//use js_sys::Date;

// Compiled by http://www.bobrosslipsum.com/ 2016 January
static PHRASES: [&str; 370] = [
    "A beautiful little sunset.",
    "A big strong tree needs big strong roots.",
    "A fan brush is a fantastic piece of equipment. Use it. Make friends with it.",
    "A happy cloud.",
    "A little happy sunlight shining through there.",
    "A thin paint will stick to a thick paint.",
    "A tree cannot be straight if it has a crooked trunk.",
    "A tree needs to be your friend if you're going to paint him.",
    "Absolutely no pressure. You are just a whisper floating across a mountain.",
    "All kinds of happy little splashes.",
    "All you have to do is let your imagination go wild.",
    "All you have to learn here is how to have fun.",
    "All you need to paint is a few tools, a little instruction, and a vision in your mind.",
    "Almost everything is going to happen for you automatically - you don't have to spend any time working or worrying.",
    "And I know you're saying, 'Oh Bob, you've done it this time.' And you may be right.",
    "And I will hypnotize that just a little bit.",
    "And just raise cain.",
    "And right there you got an almighty cloud.",
    "And that's when it becomes fun - you don't have to spend your time thinking about what's happening - you just let it happen.",
    "Any little thing can be your friend if you let it be.",
    "Anyone can paint.",
    "Anything you want to do you can do here.",
    "Anytime you learn something your time and energy are not wasted.",
    "As trees get older they lose their chlorophyll.",
    "At home you have unlimited time.",
    "Automatically, all of these beautiful, beautiful things will happen.",
    "Be brave.",
    "Be careful. You can always add more - but you can't take it away.",
    "Be so very light. Be a gentle whisper.",
    "Brown is such a nice color.",
    "But they're very easily killed. Clouds are delicate.",
    "But we're not there yet, so we don't need to worry about it.",
    "Clouds are free they come and go as they please.",
    "Clouds are free. They just float around the sky all day and have fun.",
    "Decide where your cloud lives. Maybe he lives right in here.",
    "Didn't you know you had that much power? You can move mountains. You can do anything.",
    "Do an almighty painting with us.",
    "Don't be afraid to make these big decisions. Once you start, they sort of just make themselves.",
    "Don't be bashful drop me a line.",
    "Don't fiddle with it all day.",
    "Don't fight it, use what happens.",
    "Don't forget to tell these special people in your life just how special they are to you.",
    "Don't hurry. Take your time and enjoy.",
    "Don't kill all your dark areas - you need them to show the light.",
    "Each highlight must have it's own private shadow.",
    "Even the worst thing we can do here is good.",
    "Even trees need a friend. We all need friends.",
    "Every day I learn.",
    "Every highlight needs it's own personal shadow.",
    "Every single thing in the world has its own personality - and it is up to you to make friends with the little rascals.",
    "Every time you practice, you learn more.",
    "Everybody needs a friend.",
    "Everybody's different. Trees are different. Let them all be individuals.",
    "Everyone is going to see things differently - and that's the way it should be.",
    "Everyone needs a friend. Friends are the most valuable things in the world.",
    "Everyone wants to enjoy the good parts - but you have to build the framework first.",
    "Everything is happy if you choose to make it that way.",
    "Everything's not great in life, but we can still find beauty in it.",
    "Exercising the imagination, experimenting with talents, being creative; these things, to me, are truly the windows to your soul.",
    "Fluff it up a little and hypnotize it.",
    "Fluff that up.",
    "Follow the lay of the land. It's most important.",
    "For the lack of a better word I call them hangy downs.",
    "From all of us here, I want to wish you happy painting and God bless, my friends.",
    "Get away from those little Christmas tree things we used to make in school.",
    "Get tough with it, get strong.",
    "Go out on a limb - that's where the fruit is.",
    "God gave you this gift of imagination. Use it.",
    "Happy painting, God bless.",
    "Have fun with it.",
    "Here we're limited by the time we have.",
    "Here's something that's fun.",
    "How do you make a round circle with a square knife? That's your challenge for the day.",
    "How to paint. That's easy. What to paint. That's much harder.",
    "I can't think of anything more rewarding than being able to express yourself to others through painting.",
    "I get carried away with this brush cleaning.",
    "I guess I'm a little weird. I like to talk to trees and animals. That's okay though; I have more fun than most people.",
    "I guess that would be considered a UFO. A big cotton ball in the sky.",
    "I like to beat the brush.",
    "I really believe that if you practice enough you could paint the 'Mona Lisa' with a two-inch brush.",
    "I really recommend you use odorless thinner or your spouse is gonna run you right out into the yard and you'll be working by yourself.",
    "I sincerely wish for you every possible joy life could bring.",
    "I spend a lot of time walking around in the woods and talking to trees, and squirrels, and little rabbits and stuff.",
    "I started painting as a hobby when I was little. I didn't know I had any talent. I believe talent is just a pursued interest. Anybody can do what I do.",
    "I think there's an artist hidden in the bottom of every single one of us.",
    "I thought today we would do a happy little picture.",
    "I thought today we would make a happy little stream that's just running through the woods here.",
    "I want everbody to be happy. That's what it's all about.",
    "I was blessed with a very steady hand; and it comes in very handy when you're doing these little delicate things.",
    "I will take some magic white, and a little bit of Vandyke brown and a little touch of yellow.",
    "I'll go over the colors one more time that we use: Titanium white, Thalo green, Prussian blue, Van Dyke brown, Alizarin crimson, Sap green, Cad yellow, and Permanent red.",
    "I'm a water fanatic. I love water.",
    "I'm going to mix up a little color. We'll use Van Dyke Brown, Permanent Red, and a little bit of Prussian Blue.",
    "I'm gonna start with a little Alizarin crimson and a touch of Prussian blue",
    "I'm sort of a softy, I couldn't shoot Bambi except with a camera.",
    "If I paint something, I don't want to have to explain what it is.",
    "If it's not what you want - stop and change it. Don't just keep going and expect it will get better.",
    "If there's two big trees invariably sooner or later there's gonna be a little tree.",
    "If these lines aren't straight, your water's going to run right out of your painting and get your floor wet.",
    "If we're going to have animals around we all have to be concerned about them and take care of them.",
    "If we're gonna walk though the woods, we need a little path.",
    "If what you're doing doesn't make you happy - you're doing the wrong thing.",
    "If you didn't have baby clouds, you wouldn't have big clouds.",
    "If you do too much it's going to lose its effectiveness.",
    "If you don't like it - change it. It's your world.",
    "If you don't think every day is a good day - try missing a few. You'll see.",
    "If you hypnotize it, it will go away.",
    "If you overwork it you become a cloud killer. There's nothing worse than a cloud killer.",
    "If you've been in Alaska less than a year you're a Cheechako.",
    "Imagination is the key to painting.",
    "In life you need colors.",
    "In nature, dead trees are just as normal as live trees.",
    "In painting, you have unlimited power. You have the ability to move mountains. You can bend rivers. But when I get home, the only thing I have power over is the garbage.",
    "In this world, everything can be happy.",
    "In your imagination you can go anywhere you want.",
    "In your world you can create anything you desire.",
    "In your world you have total and absolute power.",
    "Isn't it fantastic that you can change your mind and create all these happy things?",
    "Isn't it great to do something you can't fail at?",
    "Isn't that fantastic that you can create an almighty tree that fast?",
    "Isn't that fantastic?",
    "Isn't that fantastic? You can just push a little tree out of your brush like that.",
    "It is a lot of fun.",
    "It just happens - whether or not you worried about it or tried to plan it.",
    "It looks so good, I might as well not stop.",
    "It's a good way of getting rid of all your anxieties and hostilities.",
    "It's a super day, so why not make a beautiful sky?",
    "It's a very cold picture, I may have to go get my coat. It's about to freeze me to death.",
    "It's almost like something out of a fairytale book.",
    "It's amazing what you can do with a little love in your heart.",
    "It's beautiful - and we haven't even done anything to it yet.",
    "It's cold, but it's beautiful.",
    "It's hard to see things when you're too close. Take a step back and look.",
    "It's important to me that you're happy.",
    "It's life. It's interesting. It's fun.",
    "It's so important to do something every day that will make you happy.",
    "Just a happy little shadow that lives in there.",
    "Just beat the devil out of it.",
    "Just go back and put one little more happy tree in there.",
    "Just go out and talk to a tree. Make friends with it.",
    "Just let go - and fall like a little waterfall.",
    "Just let this happen. We just let this flow right out of our minds.",
    "Just let your mind wander and enjoy. This should make you happy.",
    "Just make a decision and let it go.",
    "Just make little strokes like that.",
    "Just pretend you are a whisper floating across a mountain.",
    "Just relax and let it flow. That easy.",
    "Just take out whatever you don't want. It'll change your entire perspective.",
    "Just think about these things in your mind - then bring them into your world.",
    "Just think about these things in your mind and drop em' on canvas.",
    "Just use the old one inch brush.",
    "La- da- da- da- dah. Just be happy.",
    "Learn when to stop.",
    "Let all these little things happen. Don't fight them. Learn to use them.",
    "Let all these things just sort of happen.",
    "Let that brush dance around there and play.",
    "Let the paint work.",
    "Let your heart be your guide.",
    "Let your heart take you to wherever you want to be.",
    "Let your imagination be your guide.",
    "Let your imagination be your guide.",
    "Let's build an almighty mountain.",
    "Let's build some happy little clouds up here.",
    "Let's do it again then, what the heck.",
    "Let's do that again.",
    "Let's get crazy.",
    "Let's get wild today.",
    "Let's give him a friend too. Everybody needs a friend.",
    "Let's go up in here, and start having some fun",
    "Let's have a happy little tree in here.",
    "Let's have a little bit of fun today.",
    "Let's have a nice tree right here.",
    "Let's make a happy little mountain now.",
    "Let's make a nice big leafy tree.",
    "Let's make some happy little clouds in our world.",
    "Let's make some happy little clouds in our world.",
    "Let's put a touch more of the magic here.",
    "Let's put some happy little bushes on the other side now.",
    "Let's put some happy little clouds in our world.",
    "Let's put some happy trees and bushes back in here.",
    "Let's put some highlights on these little trees. The sun wouldn't forget them.",
    "Life is too short to be alone, too precious. Share it with a friend.",
    "Little trees and bushes grow however makes them happy.",
    "Look around, look at what we have. Beauty is everywhere, you only have to look to see it.",
    "Making all those little fluffies that live in the clouds.",
    "Maybe he has a little friend that lives right over here.",
    "Maybe there was an old trapper that lived out here and maybe one day he went to check his beaver traps, and maybe he fell into the river and drowned.",
    "Maybe there's a happy little Evergreen that lives here.",
    "Maybe there's a happy little bush that lives right there.",
    "Maybe there's a happy little waterfall happening over here.",
    "Maybe there's a little something happening right here.",
    "Maybe we got a few little happy bushes here, just covered with snow.",
    "Mix your color marbly don't mix it dead.",
    "Nature is so fantastic, enjoy it. Let it make you happy.",
    "Nice little clouds playing around in the sky.",
    "Nice little fluffy clouds laying around in the sky being lazy.",
    "No pressure. Just relax and watch it happen.",
    "No worries. No cares. Just float and wait for the wind to blow you around.",
    "Nothing wrong with washing your brush.",
    "Nothing's gonna make your husband or wife madder than coming home and having a snow-covered dinner.",
    "Now it's beginning to make a little sense.",
    "Now let's put some happy little clouds in here.",
    "Now then, let's play.",
    "Now we can begin working on lots of happy little things.",
    "Now we don't want him to get lonely, so we'll give him a little friend.",
    "Now we'll take the almighty fan brush.",
    "Now, we're going to fluff this cloud.",
    "Of course he's a happy little stone, cause we don't have any other kind.",
    "Once you learn the technique, ohhh! Turn you loose on the world; you become a tiger.",
    "Only God can make a tree - but you can paint one.",
    "Only eight colors that you need.",
    "Only think about one thing at a time. Don't get greedy.",
    "Paint anything you want on the canvas. Create your own world.",
    "Painting should do one thing. It should put happiness in your heart.",
    "Poor old tree.",
    "Pretend you're water. Just floating without any effort. Having a good day.",
    "Put it in, leave it alone.",
    "Put light against light - you have nothing. Put dark against dark - you have nothing. It's the contrast of light and dark that each give the other one meaning.",
    "Put your feelings into it, your heart, it's your world.",
    "Remember how free clouds are. They just lay around in the sky all day long.",
    "See how easy it is to create a little tree right in your world.",
    "See there how easy that is.",
    "See there, told you that would be easy.",
    "See. We take the corner of the brush and let it play back-and-forth.",
    "So often we avoid running water, and running water is a lot of fun.",
    "Sometimes you learn more from your mistakes than you do from your masterpieces.",
    "Son of a gun.",
    "Steve wants reflections, so let's give him reflections.",
    "Take your time. Speed will come later.",
    "Talent is a pursued interest. That is to say, anything you practice you can do.",
    "Talk to trees, look at the birds. Whatever it takes.",
    "That is when you can experience true joy, when you have no fear.",
    "That's a crooked tree. We'll send him to Washington.",
    "That's a son of a gun of a cloud.",
    "That's crazy.",
    "That's the way I look when I get home late; black and blue.",
    "That's what makes life fun. That you can make these decisions. That you can create the world that you want.",
    "That's what painting is all about. It should make you feel good when you paint.",
    "That's why I paint - because I can create the kind of world I want - and I can make this world as happy as I want it.",
    "The first step to doing anything is to believe you can do it. See it finished in your mind before you ever start.",
    "The least little bit can do so much.",
    "The light is your friend. Preserve it.",
    "The little tiny Tim easels will let you down.",
    "The man who does the best job is the one who is happy at his job.",
    "The more we do this - the more it will do good things to our heart.",
    "The only prerequisite is that it makes you happy. If it makes you happy then it's good.",
    "The only thing worse than yellow snow is green snow.",
    "The secret to doing anything is believing that you can do it. Anything that you believe you can do strong enough, you can do. Anything. As long as you believe.",
    "The shadows are just like the highlights, but we're going in the opposite direction.",
    "The very fact that you're aware of suffering is enough reason to be overjoyed that you're alive and can experience it.",
    "There are no limits in this world.",
    "There are no mistakes. You can fix anything that happens.",
    "There comes a nice little fluffer.",
    "There is immense joy in just watching - watching all the little creatures in nature.",
    "There is no right or wrong - as long as it makes you happy and doesn't hurt anyone.",
    "There isn't a rule. You just practice and find out which way works best for you.",
    "There it is.",
    "There we go.",
    "There's not a thing in the world wrong with washing your brush.",
    "There's nothing wrong with having a tree as a friend.",
    "These little son of a guns hide in your brush and you just have to push them out.",
    "These things happen automatically. All you have to do is just let them happen.",
    "These trees are so much fun. I get started on them and I have a hard time stopping.",
    "They say everything looks better with odd numbers of things. But sometimes I put even numbers--just to upset the critics.",
    "Think about a cloud. Just float around and be there.",
    "This is a happy place, little squirrels live here and play.",
    "This is an example of what you can do with just a few things, a little imagination and a happy dream in your heart.",
    "This is gonna be a happy little seascape.",
    "This is probably the greatest thing that's ever happened in my life.",
    "This is probably the greatest thing to happen in my life - to be able to share this with you.",
    "This is the fun part",
    "This is the time to get out all your flustrations, much better than kicking the dog around the house or taking it out on your spouse.",
    "This is the way you take out your flustrations.",
    "This is truly an almighty mountain.",
    "This is unplanned it really just happens.",
    "This is where you take out all your hostilities and frustrations. It's better than kicking the puppy dog around and all that so.",
    "This is your creation - and it's just as unique and special as you are.",
    "This is your world, whatever makes you happy you can put in it. Go crazy.",
    "This is your world.",
    "This painting comes right out of your heart.",
    "This piece of canvas is your world.",
    "This present moment is perfect simply due to the fact you're experiencing it.",
    "Those great big fluffy clouds.",
    "Tree trunks grow however makes them happy.",
    "Trees cover up a multitude of sins.",
    "Trees get lonely too, so we'll give him a little friend.",
    "Trees grow however makes them happy.",
    "Trees grow in all kinds of ways. They're not all perfectly straight. Not every limb is perfect.",
    "Trees live in your fan brush, but you have to scare them out.",
    "Use absolutely no pressure. Just like an angel's wing.",
    "Use what happens naturally, don't fight it.",
    "Use what you see, don't plan it.",
    "Use your imagination, let it go.",
    "Van Dyke Brown is a very nice brown, it's almost like a chocolate brown.",
    "Very easy to work these to death.",
    "Volunteering your time; it pays you and your whole community fantastic dividends.",
    "Water's like me. It's laaazy... Boy, it always looks for the easiest way to do things",
    "We artists are a different breed of people. We're a happy bunch.",
    "We can always carry this a step further. There's really no end to this.",
    "We can fix anything.",
    "We don't have anything but happy trees here.",
    "We don't have to be committed. We are just playing here.",
    "We don't have to be concerned about it. We just have to let it fall where it will.",
    "We don't make mistakes we just have happy little accidents.",
    "We don't need any guidelines or formats. All we need to do is just let it flow right out of us.",
    "We don't really know where this goes - and I'm not sure we really care.",
    "We don't want to set these clouds on fire.",
    "We have a fantastic little sky!",
    "We have all at one time or another mixed some mud.",
    "We have no limits to our world. We're only limited by our imagination.",
    "We might as well make some Almighty mountains today as well, what the heck.",
    "We must be quiet, soft and gentle.",
    "We need dark in order to show light.",
    "We spend so much of our life looking - but never seeing.",
    "We start with a vision in our heart, and we put it one canvas.",
    "We tell people sometimes: we're like drug dealers, come into town and get everybody absolutely addicted to painting. It doesn't take much to get you addicted.",
    "We want to use a lot pressure while using no pressure at all.",
    "We wash our brush with odorless thinner.",
    "We'll do another happy little painting.",
    "We'll have a super time.",
    "We'll lay all these little funky little things in there.",
    "We'll make some happy little bushes here.",
    "We'll paint one happy little tree right here.",
    "We'll play with clouds today.",
    "We'll put all the little clouds in and let them dance around and have fun.",
    "We'll put some happy little leaves here and there.",
    "We'll take a little bit of Van Dyke Brown.",
    "We'll throw some old gray clouds in here just sneaking around and having fun.",
    "We're not trying to teach you a thing to copy. We're just here to teach you a technique, then let you loose into the world.",
    "We're trying to teach you a technique here and how to use it.",
    "What the devil.",
    "When things happen - enjoy them. They're little gifts.",
    "When you buy that first tube of paint it gives you an artist license.",
    "When you do it your way you can go anywhere you choose.",
    "With practice comes confidence.",
    "With something so strong, a little bit can go a long way.",
    "Work on one thing at a time. Don't get carried away - we have plenty of time.",
    "Work that paint.",
    "Working it up and down, back and forth.",
    "You are only limited by your imagination.",
    "You better get your coat out, this is going to be a cold painting.",
    "You can create anything that makes you happy.",
    "You can create beautiful things - but you have to see them in your mind first.",
    "You can create the world you want to see and be a part of. You have that power.",
    "You can do anything here - the only prerequisite is that it makes you happy.",
    "You can do anything here. So don't worry about it.",
    "You can do anything your heart can imagine.",
    "You can do it.",
    "You can get away with a lot.",
    "You can spend all day playing with mountains.",
    "You can work and carry-on and put lots of little happy things in here.",
    "You can't have light without dark. You can't know happiness unless you've known sorrow.",
    "You can't make a mistake. Anything that happens you can learn to use - and make something beautiful out of it.",
    "You could sit here for weeks with your one hair brush trying to do that - or you could do it with one stroke with an almighty brush.",
    "You create the dream - then you bring it into your world.",
    "You don't have to be crazy to do this but it does help.",
    "You don't have to spend all your time thinking about what you're doing, you just let it happen.",
    "You don't want to kill all your dark areas they are very important.",
    "You got your heavy coat out yet? It's getting colder.",
    "You gotta think like a tree.",
    "You have freedom here. The only guide is your heart.",
    "You have to allow the paint to break to make it beautiful.",
    "You have to make almighty decisions when you're the creator.",
    "You have to make these big decisions.",
    "You have to make those little noises or it won't work.",
    "You need the dark in order to show the light.",
    "You want your tree to have some character. Make it special.",
    "You're meant to have fun in life.",
    "You're the greatest thing that has ever been or ever will be. You're special. You're so very special.",
    "You've got to learn to fight the temptation to resist these things. Just let them happen."
];

fn get_random_indexes(cnt: usize) -> Vec<usize> {
    let mut rng = get_rng();
    let range = Range::new(0, PHRASES.len());    
    (0..cnt)
        .map(|_| range.sample(&mut rng))
        .collect()
}

fn get_phrase(idx: usize) -> &'static str {
    PHRASES[idx]        
}

fn get_rng() -> SmallRng {    
    //No sys calls available. Use the Javascript implementation to get the ticks
    // let ticks = Date::now();  
    // Prng::with(SeedType::External(ticks as u32))
   // SmallRng::from_entropy()

    use js_sys::Date;
    use rand::SeedableRng;

    let ticks = Date::now(); 
    //convert the number to byte array
    let tick_bytes = transmute(ticks as u128); 
    SmallRng::from_seed(tick_bytes)
}

fn transmute(x: u128) -> [u8; 16] {

        let b1 : u8 = ((x >> 120) & 0xffffffff) as u8;
        let b2 : u8 = ((x >> 112) & 0xffffffff) as u8;
        let b3 : u8 = ((x >> 104) & 0xffffffff) as u8;
        let b4 : u8 = ((x >> 96) & 0xffffffff) as u8;    
        
        let b5 : u8 = ((x >> 88) & 0xffffffff) as u8;
        let b6 : u8 = ((x >> 80) & 0xffffffff) as u8;
        let b7 : u8 = ((x >> 72) & 0xffffffff) as u8;    
        let b8 : u8 = ((x >> 64) & 0xffffffff) as u8;    

        let b9 : u8 = ((x >> 56) & 0xffffffff) as u8;
        let b10 : u8 = ((x >> 48) & 0xffffffff) as u8;
        let b11 : u8 = ((x >> 40) & 0xffffffff) as u8;
        let b12 : u8 = ((x >> 32) & 0xffffffff) as u8;
        
        let b13 : u8 = ((x >> 24) & 0xffffffff) as u8;
        let b14 : u8 = ((x >> 16) & 0xffffffff) as u8;
        let b15 : u8 = ((x >> 8) & 0xffffffff) as u8;    
        let b16 : u8 = (x & 0xffffffff) as u8;

        //Most of the entropy is in the last few bytes and generators are allowed
        //to assume evenly spread entropy in the seed, so spread the bytes around
        [b16, b1, b14, b3, b12, b5, b10, b7, b8, b9, b6, b11, b4, b13, b2, b15]
    }

fn get_phrases(idxs: &Vec<usize>) -> Vec<&'static str> {    
    idxs.iter()
        .map(|idx| get_phrase(*idx))
        .collect()
}

fn need_newline(newline: usize, idx: usize) -> bool {
    //idx+1 because idx is zero-based but we want a new line after "every x phrases".
    (newline > 0) && (idx > 0) && ((idx + 1) % newline == 0)
}

fn need_space(newline: usize, idx: usize) -> bool {
    !need_newline(newline, idx)
}

fn build_phrase_text(idxs: Vec<usize>, newline: usize) -> String {
    let phrases_vec = get_phrases(&idxs);
    let mut string = String::new();
    for i in 0..phrases_vec.len() {
        //the phrase
        string.push_str(phrases_vec[i]);
        //spaces between phrases
        if need_space(newline, i) {
            string.push(' ');
        }
        //new lines
        if need_newline(newline, i) {
            string.push_str("\n\n");
        }
    }
    string
}

pub fn get_phrase_text(phrase_cnt: usize, newline: usize) -> String {
    let idxs = get_random_indexes(phrase_cnt);
    build_phrase_text(idxs,newline)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_indexes() -> Vec<usize> {
        vec![34, 2, 99, 43, 128, 300, 45, 56, 303, 42, 11]
    }
    
    #[test]
    fn get_phrases() {
        let randoms = get_test_indexes();
        let phrases = super::get_phrases(&randoms);
        println!("{:?}", phrases);
    }
}