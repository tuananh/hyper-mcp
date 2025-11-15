// Embed templates.json
pub const TEMPLATES_JSON: &str = include_str!("../templates.json");

// Embed font data
pub const FONT_DATA: &[u8] = include_bytes!("../assets/fonts/NotoSans-Bold.ttf");

// Function to get template config
pub fn get_template_config(template_id: &str) -> Option<&'static str> {
    match template_id {
        "aag" => Some(include_str!("../assets/templates/aag/config.yml")),
        "ackbar" => Some(include_str!("../assets/templates/ackbar/config.yml")),
        "afraid" => Some(include_str!("../assets/templates/afraid/config.yml")),
        "agnes" => Some(include_str!("../assets/templates/agnes/config.yml")),
        "aint-got-time" => Some(include_str!("../assets/templates/aint-got-time/config.yml")),
        "ams" => Some(include_str!("../assets/templates/ams/config.yml")),
        "ants" => Some(include_str!("../assets/templates/ants/config.yml")),
        "apcr" => Some(include_str!("../assets/templates/apcr/config.yml")),
        "astronaut" => Some(include_str!("../assets/templates/astronaut/config.yml")),
        "atis" => Some(include_str!("../assets/templates/atis/config.yml")),
        "away" => Some(include_str!("../assets/templates/away/config.yml")),
        "awesome" => Some(include_str!("../assets/templates/awesome/config.yml")),
        "awesome-awkward" => Some(include_str!(
            "../assets/templates/awesome-awkward/config.yml"
        )),
        "awkward" => Some(include_str!("../assets/templates/awkward/config.yml")),
        "awkward-awesome" => Some(include_str!(
            "../assets/templates/awkward-awesome/config.yml"
        )),
        "bad" => Some(include_str!("../assets/templates/bad/config.yml")),
        "badchoice" => Some(include_str!("../assets/templates/badchoice/config.yml")),
        "balloon" => Some(include_str!("../assets/templates/balloon/config.yml")),
        "bd" => Some(include_str!("../assets/templates/bd/config.yml")),
        "because" => Some(include_str!("../assets/templates/because/config.yml")),
        "bender" => Some(include_str!("../assets/templates/bender/config.yml")),
        "bihw" => Some(include_str!("../assets/templates/bihw/config.yml")),
        "bilbo" => Some(include_str!("../assets/templates/bilbo/config.yml")),
        "biw" => Some(include_str!("../assets/templates/biw/config.yml")),
        "blb" => Some(include_str!("../assets/templates/blb/config.yml")),
        "boat" => Some(include_str!("../assets/templates/boat/config.yml")),
        "bongo" => Some(include_str!("../assets/templates/bongo/config.yml")),
        "both" => Some(include_str!("../assets/templates/both/config.yml")),
        "box" => Some(include_str!("../assets/templates/box/config.yml")),
        "bs" => Some(include_str!("../assets/templates/bs/config.yml")),
        "bus" => Some(include_str!("../assets/templates/bus/config.yml")),
        "buzz" => Some(include_str!("../assets/templates/buzz/config.yml")),
        "cake" => Some(include_str!("../assets/templates/cake/config.yml")),
        "captain" => Some(include_str!("../assets/templates/captain/config.yml")),
        "captain-america" => Some(include_str!(
            "../assets/templates/captain-america/config.yml"
        )),
        "cb" => Some(include_str!("../assets/templates/cb/config.yml")),
        "cbb" => Some(include_str!("../assets/templates/cbb/config.yml")),
        "cbg" => Some(include_str!("../assets/templates/cbg/config.yml")),
        "center" => Some(include_str!("../assets/templates/center/config.yml")),
        "ch" => Some(include_str!("../assets/templates/ch/config.yml")),
        "chair" => Some(include_str!("../assets/templates/chair/config.yml")),
        "cheems" => Some(include_str!("../assets/templates/cheems/config.yml")),
        "chosen" => Some(include_str!("../assets/templates/chosen/config.yml")),
        "cmm" => Some(include_str!("../assets/templates/cmm/config.yml")),
        "country" => Some(include_str!("../assets/templates/country/config.yml")),
        "crazypills" => Some(include_str!("../assets/templates/crazypills/config.yml")),
        "crow" => Some(include_str!("../assets/templates/crow/config.yml")),
        "cryingfloor" => Some(include_str!("../assets/templates/cryingfloor/config.yml")),
        "db" => Some(include_str!("../assets/templates/db/config.yml")),
        "dbg" => Some(include_str!("../assets/templates/dbg/config.yml")),
        "dg" => Some(include_str!("../assets/templates/dg/config.yml")),
        "disastergirl" => Some(include_str!("../assets/templates/disastergirl/config.yml")),
        "dodgson" => Some(include_str!("../assets/templates/dodgson/config.yml")),
        "doge" => Some(include_str!("../assets/templates/doge/config.yml")),
        "dragon" => Some(include_str!("../assets/templates/dragon/config.yml")),
        "drake" => Some(include_str!("../assets/templates/drake/config.yml")),
        "drowning" => Some(include_str!("../assets/templates/drowning/config.yml")),
        "drunk" => Some(include_str!("../assets/templates/drunk/config.yml")),
        "ds" => Some(include_str!("../assets/templates/ds/config.yml")),
        "dsm" => Some(include_str!("../assets/templates/dsm/config.yml")),
        "dwight" => Some(include_str!("../assets/templates/dwight/config.yml")),
        "elf" => Some(include_str!("../assets/templates/elf/config.yml")),
        "elmo" => Some(include_str!("../assets/templates/elmo/config.yml")),
        "ermg" => Some(include_str!("../assets/templates/ermg/config.yml")),
        "exit" => Some(include_str!("../assets/templates/exit/config.yml")),
        "fa" => Some(include_str!("../assets/templates/fa/config.yml")),
        "facepalm" => Some(include_str!("../assets/templates/facepalm/config.yml")),
        "fbf" => Some(include_str!("../assets/templates/fbf/config.yml")),
        "feelsgood" => Some(include_str!("../assets/templates/feelsgood/config.yml")),
        "fetch" => Some(include_str!("../assets/templates/fetch/config.yml")),
        "fine" => Some(include_str!("../assets/templates/fine/config.yml")),
        "firsttry" => Some(include_str!("../assets/templates/firsttry/config.yml")),
        "fmr" => Some(include_str!("../assets/templates/fmr/config.yml")),
        "friends" => Some(include_str!("../assets/templates/friends/config.yml")),
        "fry" => Some(include_str!("../assets/templates/fry/config.yml")),
        "fwp" => Some(include_str!("../assets/templates/fwp/config.yml")),
        "gandalf" => Some(include_str!("../assets/templates/gandalf/config.yml")),
        "gb" => Some(include_str!("../assets/templates/gb/config.yml")),
        "gears" => Some(include_str!("../assets/templates/gears/config.yml")),
        "genie" => Some(include_str!("../assets/templates/genie/config.yml")),
        "ggg" => Some(include_str!("../assets/templates/ggg/config.yml")),
        "glasses" => Some(include_str!("../assets/templates/glasses/config.yml")),
        "gone" => Some(include_str!("../assets/templates/gone/config.yml")),
        "grave" => Some(include_str!("../assets/templates/grave/config.yml")),
        "gru" => Some(include_str!("../assets/templates/gru/config.yml")),
        "grumpycat" => Some(include_str!("../assets/templates/grumpycat/config.yml")),
        "hagrid" => Some(include_str!("../assets/templates/hagrid/config.yml")),
        "happening" => Some(include_str!("../assets/templates/happening/config.yml")),
        "harold" => Some(include_str!("../assets/templates/harold/config.yml")),
        "headaches" => Some(include_str!("../assets/templates/headaches/config.yml")),
        "hipster" => Some(include_str!("../assets/templates/hipster/config.yml")),
        "home" => Some(include_str!("../assets/templates/home/config.yml")),
        "icanhas" => Some(include_str!("../assets/templates/icanhas/config.yml")),
        "imsorry" => Some(include_str!("../assets/templates/imsorry/config.yml")),
        "inigo" => Some(include_str!("../assets/templates/inigo/config.yml")),
        "interesting" => Some(include_str!("../assets/templates/interesting/config.yml")),
        "ive" => Some(include_str!("../assets/templates/ive/config.yml")),
        "iw" => Some(include_str!("../assets/templates/iw/config.yml")),
        "jd" => Some(include_str!("../assets/templates/jd/config.yml")),
        "jetpack" => Some(include_str!("../assets/templates/jetpack/config.yml")),
        "jim" => Some(include_str!("../assets/templates/jim/config.yml")),
        "joker" => Some(include_str!("../assets/templates/joker/config.yml")),
        "jw" => Some(include_str!("../assets/templates/jw/config.yml")),
        "keanu" => Some(include_str!("../assets/templates/keanu/config.yml")),
        "kermit" => Some(include_str!("../assets/templates/kermit/config.yml")),
        "khaby-lame" => Some(include_str!("../assets/templates/khaby-lame/config.yml")),
        "kk" => Some(include_str!("../assets/templates/kk/config.yml")),
        "kombucha" => Some(include_str!("../assets/templates/kombucha/config.yml")),
        "kramer" => Some(include_str!("../assets/templates/kramer/config.yml")),
        "leo" => Some(include_str!("../assets/templates/leo/config.yml")),
        "light" => Some(include_str!("../assets/templates/light/config.yml")),
        "live" => Some(include_str!("../assets/templates/live/config.yml")),
        "ll" => Some(include_str!("../assets/templates/ll/config.yml")),
        "lrv" => Some(include_str!("../assets/templates/lrv/config.yml")),
        "made" => Some(include_str!("../assets/templates/made/config.yml")),
        "mb" => Some(include_str!("../assets/templates/mb/config.yml")),
        "michael-scott" => Some(include_str!("../assets/templates/michael-scott/config.yml")),
        "midwit" => Some(include_str!("../assets/templates/midwit/config.yml")),
        "millers" => Some(include_str!("../assets/templates/millers/config.yml")),
        "mini-keanu" => Some(include_str!("../assets/templates/mini-keanu/config.yml")),
        "mmm" => Some(include_str!("../assets/templates/mmm/config.yml")),
        "money" => Some(include_str!("../assets/templates/money/config.yml")),
        "mordor" => Some(include_str!("../assets/templates/mordor/config.yml")),
        "morpheus" => Some(include_str!("../assets/templates/morpheus/config.yml")),
        "mouth" => Some(include_str!("../assets/templates/mouth/config.yml")),
        "mw" => Some(include_str!("../assets/templates/mw/config.yml")),
        "nails" => Some(include_str!("../assets/templates/nails/config.yml")),
        "nice" => Some(include_str!("../assets/templates/nice/config.yml")),
        "noah" => Some(include_str!("../assets/templates/noah/config.yml")),
        "noidea" => Some(include_str!("../assets/templates/noidea/config.yml")),
        "ntot" => Some(include_str!("../assets/templates/ntot/config.yml")),
        "oag" => Some(include_str!("../assets/templates/oag/config.yml")),
        "officespace" => Some(include_str!("../assets/templates/officespace/config.yml")),
        "older" => Some(include_str!("../assets/templates/older/config.yml")),
        "oprah" => Some(include_str!("../assets/templates/oprah/config.yml")),
        "panik-kalm-panik" => Some(include_str!(
            "../assets/templates/panik-kalm-panik/config.yml"
        )),
        "patrick" => Some(include_str!("../assets/templates/patrick/config.yml")),
        "perfection" => Some(include_str!("../assets/templates/perfection/config.yml")),
        "persian" => Some(include_str!("../assets/templates/persian/config.yml")),
        "philosoraptor" => Some(include_str!("../assets/templates/philosoraptor/config.yml")),
        "pigeon" => Some(include_str!("../assets/templates/pigeon/config.yml")),
        "pooh" => Some(include_str!("../assets/templates/pooh/config.yml")),
        "pool" => Some(include_str!("../assets/templates/pool/config.yml")),
        "prop3" => Some(include_str!("../assets/templates/prop3/config.yml")),
        "ptj" => Some(include_str!("../assets/templates/ptj/config.yml")),
        "puffin" => Some(include_str!("../assets/templates/puffin/config.yml")),
        "red" => Some(include_str!("../assets/templates/red/config.yml")),
        "regret" => Some(include_str!("../assets/templates/regret/config.yml")),
        "remembers" => Some(include_str!("../assets/templates/remembers/config.yml")),
        "reveal" => Some(include_str!("../assets/templates/reveal/config.yml")),
        "right" => Some(include_str!("../assets/templates/right/config.yml")),
        "rollsafe" => Some(include_str!("../assets/templates/rollsafe/config.yml")),
        "sad-biden" => Some(include_str!("../assets/templates/sad-biden/config.yml")),
        "sad-boehner" => Some(include_str!("../assets/templates/sad-boehner/config.yml")),
        "sad-bush" => Some(include_str!("../assets/templates/sad-bush/config.yml")),
        "sad-clinton" => Some(include_str!("../assets/templates/sad-clinton/config.yml")),
        "sad-obama" => Some(include_str!("../assets/templates/sad-obama/config.yml")),
        "sadfrog" => Some(include_str!("../assets/templates/sadfrog/config.yml")),
        "saltbae" => Some(include_str!("../assets/templates/saltbae/config.yml")),
        "same" => Some(include_str!("../assets/templates/same/config.yml")),
        "sarcasticbear" => Some(include_str!("../assets/templates/sarcasticbear/config.yml")),
        "say" => Some(include_str!("../assets/templates/say/config.yml")),
        "sb" => Some(include_str!("../assets/templates/sb/config.yml")),
        "scc" => Some(include_str!("../assets/templates/scc/config.yml")),
        "seagull" => Some(include_str!("../assets/templates/seagull/config.yml")),
        "sf" => Some(include_str!("../assets/templates/sf/config.yml")),
        "sk" => Some(include_str!("../assets/templates/sk/config.yml")),
        "ski" => Some(include_str!("../assets/templates/ski/config.yml")),
        "slap" => Some(include_str!("../assets/templates/slap/config.yml")),
        "snek" => Some(include_str!("../assets/templates/snek/config.yml")),
        "soa" => Some(include_str!("../assets/templates/soa/config.yml")),
        "sohappy" => Some(include_str!("../assets/templates/sohappy/config.yml")),
        "sohot" => Some(include_str!("../assets/templates/sohot/config.yml")),
        "soup-nazi" => Some(include_str!("../assets/templates/soup-nazi/config.yml")),
        "sparta" => Some(include_str!("../assets/templates/sparta/config.yml")),
        "spiderman" => Some(include_str!("../assets/templates/spiderman/config.yml")),
        "spirit" => Some(include_str!("../assets/templates/spirit/config.yml")),
        "spongebob" => Some(include_str!("../assets/templates/spongebob/config.yml")),
        "ss" => Some(include_str!("../assets/templates/ss/config.yml")),
        "stew" => Some(include_str!("../assets/templates/stew/config.yml")),
        "stonks" => Some(include_str!("../assets/templates/stonks/config.yml")),
        "stop" => Some(include_str!("../assets/templates/stop/config.yml")),
        "stop-it" => Some(include_str!("../assets/templates/stop-it/config.yml")),
        "success" => Some(include_str!("../assets/templates/success/config.yml")),
        "tenguy" => Some(include_str!("../assets/templates/tenguy/config.yml")),
        "toohigh" => Some(include_str!("../assets/templates/toohigh/config.yml")),
        "touch" => Some(include_str!("../assets/templates/touch/config.yml")),
        "tried" => Some(include_str!("../assets/templates/tried/config.yml")),
        "trump" => Some(include_str!("../assets/templates/trump/config.yml")),
        "ugandanknuck" => Some(include_str!("../assets/templates/ugandanknuck/config.yml")),
        "vince" => Some(include_str!("../assets/templates/vince/config.yml")),
        "wallet" => Some(include_str!("../assets/templates/wallet/config.yml")),
        "waygd" => Some(include_str!("../assets/templates/waygd/config.yml")),
        "wddth" => Some(include_str!("../assets/templates/wddth/config.yml")),
        "whatyear" => Some(include_str!("../assets/templates/whatyear/config.yml")),
        "winter" => Some(include_str!("../assets/templates/winter/config.yml")),
        "wishes" => Some(include_str!("../assets/templates/wishes/config.yml")),
        "wkh" => Some(include_str!("../assets/templates/wkh/config.yml")),
        "woman-cat" => Some(include_str!("../assets/templates/woman-cat/config.yml")),
        "wonka" => Some(include_str!("../assets/templates/wonka/config.yml")),
        "worst" => Some(include_str!("../assets/templates/worst/config.yml")),
        "xy" => Some(include_str!("../assets/templates/xy/config.yml")),
        "yallgot" => Some(include_str!("../assets/templates/yallgot/config.yml")),
        "yodawg" => Some(include_str!("../assets/templates/yodawg/config.yml")),
        "yuno" => Some(include_str!("../assets/templates/yuno/config.yml")),
        "zero-wing" => Some(include_str!("../assets/templates/zero-wing/config.yml")),
        _ => None,
    }
}

// Function to get template image
pub fn get_template_image(template_id: &str, image_name: &str) -> Option<&'static [u8]> {
    match (template_id, image_name) {
        ("aag", "default.jpg") => Some(include_bytes!("../assets/templates/aag/default.jpg")),
        ("ackbar", "default.jpg") => Some(include_bytes!("../assets/templates/ackbar/default.jpg")),
        ("afraid", "default.jpg") => Some(include_bytes!("../assets/templates/afraid/default.jpg")),
        ("agnes", "default.jpg") => Some(include_bytes!("../assets/templates/agnes/default.jpg")),
        ("aint-got-time", "default.jpg") => Some(include_bytes!(
            "../assets/templates/aint-got-time/default.jpg"
        )),
        ("ams", "default.jpg") => Some(include_bytes!("../assets/templates/ams/default.jpg")),
        ("ants", "default.jpg") => Some(include_bytes!("../assets/templates/ants/default.jpg")),
        ("apcr", "default.jpg") => Some(include_bytes!("../assets/templates/apcr/default.jpg")),
        ("astronaut", "default.png") => {
            Some(include_bytes!("../assets/templates/astronaut/default.png"))
        }
        ("atis", "default.jpg") => Some(include_bytes!("../assets/templates/atis/default.jpg")),
        ("away", "default.jpg") => Some(include_bytes!("../assets/templates/away/default.jpg")),
        ("awesome", "default.jpg") => {
            Some(include_bytes!("../assets/templates/awesome/default.jpg"))
        }
        ("awesome-awkward", "default.jpg") => Some(include_bytes!(
            "../assets/templates/awesome-awkward/default.jpg"
        )),
        ("awkward", "default.jpg") => {
            Some(include_bytes!("../assets/templates/awkward/default.jpg"))
        }
        ("awkward-awesome", "default.jpg") => Some(include_bytes!(
            "../assets/templates/awkward-awesome/default.jpg"
        )),
        ("bad", "default.jpg") => Some(include_bytes!("../assets/templates/bad/default.jpg")),
        ("badchoice", "default.jpg") => {
            Some(include_bytes!("../assets/templates/badchoice/default.jpg"))
        }
        ("balloon", "default.jpg") => {
            Some(include_bytes!("../assets/templates/balloon/default.jpg"))
        }
        ("bd", "default.jpg") => Some(include_bytes!("../assets/templates/bd/default.jpg")),
        ("because", "default.png") => {
            Some(include_bytes!("../assets/templates/because/default.png"))
        }
        ("bender", "default.jpg") => Some(include_bytes!("../assets/templates/bender/default.jpg")),
        ("bihw", "default.jpg") => Some(include_bytes!("../assets/templates/bihw/default.jpg")),
        ("bilbo", "default.jpg") => Some(include_bytes!("../assets/templates/bilbo/default.jpg")),
        ("biw", "default.jpg") => Some(include_bytes!("../assets/templates/biw/default.jpg")),
        ("blb", "default.jpg") => Some(include_bytes!("../assets/templates/blb/default.jpg")),
        ("boat", "default.jpg") => Some(include_bytes!("../assets/templates/boat/default.jpg")),
        ("bongo", "default.gif") => Some(include_bytes!("../assets/templates/bongo/default.gif")),
        ("both", "default.jpg") => Some(include_bytes!("../assets/templates/both/default.jpg")),
        ("both", "default.gif") => Some(include_bytes!("../assets/templates/both/default.gif")),
        ("box", "default.png") => Some(include_bytes!("../assets/templates/box/default.png")),
        ("box", "default.gif") => Some(include_bytes!("../assets/templates/box/default.gif")),
        ("bs", "default.jpg") => Some(include_bytes!("../assets/templates/bs/default.jpg")),
        ("bus", "default.jpg") => Some(include_bytes!("../assets/templates/bus/default.jpg")),
        ("buzz", "default.jpg") => Some(include_bytes!("../assets/templates/buzz/default.jpg")),
        ("buzz", "default.gif") => Some(include_bytes!("../assets/templates/buzz/default.gif")),
        ("cake", "default.jpg") => Some(include_bytes!("../assets/templates/cake/default.jpg")),
        ("cake", "default.gif") => Some(include_bytes!("../assets/templates/cake/default.gif")),
        ("captain", "default.jpg") => {
            Some(include_bytes!("../assets/templates/captain/default.jpg"))
        }
        ("captain-america", "default.jpg") => Some(include_bytes!(
            "../assets/templates/captain-america/default.jpg"
        )),
        ("cb", "default.jpg") => Some(include_bytes!("../assets/templates/cb/default.jpg")),
        ("cbb", "default.jpg") => Some(include_bytes!("../assets/templates/cbb/default.jpg")),
        ("cbg", "default.jpg") => Some(include_bytes!("../assets/templates/cbg/default.jpg")),
        ("center", "default.jpg") => Some(include_bytes!("../assets/templates/center/default.jpg")),
        ("ch", "default.jpg") => Some(include_bytes!("../assets/templates/ch/default.jpg")),
        ("chair", "default.png") => Some(include_bytes!("../assets/templates/chair/default.png")),
        ("cheems", "default.jpg") => Some(include_bytes!("../assets/templates/cheems/default.jpg")),
        ("chosen", "default.jpg") => Some(include_bytes!("../assets/templates/chosen/default.jpg")),
        ("cmm", "default.png") => Some(include_bytes!("../assets/templates/cmm/default.png")),
        ("country", "default.jpg") => {
            Some(include_bytes!("../assets/templates/country/default.jpg"))
        }
        ("crazypills", "default.png") => {
            Some(include_bytes!("../assets/templates/crazypills/default.png"))
        }
        ("crow", "default.jpg") => Some(include_bytes!("../assets/templates/crow/default.jpg")),
        ("cryingfloor", "default.jpg") => Some(include_bytes!(
            "../assets/templates/cryingfloor/default.jpg"
        )),
        ("db", "default.jpg") => Some(include_bytes!("../assets/templates/db/default.jpg")),
        ("dbg", "default.jpg") => Some(include_bytes!("../assets/templates/dbg/default.jpg")),
        ("dg", "default.jpg") => Some(include_bytes!("../assets/templates/dg/default.jpg")),
        ("disastergirl", "default.jpg") => Some(include_bytes!(
            "../assets/templates/disastergirl/default.jpg"
        )),
        ("dodgson", "default.jpg") => {
            Some(include_bytes!("../assets/templates/dodgson/default.jpg"))
        }
        ("dodgson", "default.gif") => {
            Some(include_bytes!("../assets/templates/dodgson/default.gif"))
        }
        ("doge", "bark.jpg") => Some(include_bytes!("../assets/templates/doge/bark.jpg")),
        ("doge", "pet.jpg") => Some(include_bytes!("../assets/templates/doge/pet.jpg")),
        ("doge", "growl.jpg") => Some(include_bytes!("../assets/templates/doge/growl.jpg")),
        ("doge", "roll.jpg") => Some(include_bytes!("../assets/templates/doge/roll.jpg")),
        ("doge", "default.jpg") => Some(include_bytes!("../assets/templates/doge/default.jpg")),
        ("doge", "bite.jpg") => Some(include_bytes!("../assets/templates/doge/bite.jpg")),
        ("doge", "full.jpg") => Some(include_bytes!("../assets/templates/doge/full.jpg")),
        ("dragon", "default.png") => Some(include_bytes!("../assets/templates/dragon/default.png")),
        ("drake", "beat.jpg") => Some(include_bytes!("../assets/templates/drake/beat.jpg")),
        ("drake", "yes.jpg") => Some(include_bytes!("../assets/templates/drake/yes.jpg")),
        ("drake", "default.png") => Some(include_bytes!("../assets/templates/drake/default.png")),
        ("drake", "padding.jpg") => Some(include_bytes!("../assets/templates/drake/padding.jpg")),
        ("drake", "no.jpg") => Some(include_bytes!("../assets/templates/drake/no.jpg")),
        ("drowning", "default.png") => {
            Some(include_bytes!("../assets/templates/drowning/default.png"))
        }
        ("drunk", "default.jpg") => Some(include_bytes!("../assets/templates/drunk/default.jpg")),
        ("ds", "maga.jpg") => Some(include_bytes!("../assets/templates/ds/maga.jpg")),
        ("ds", "default.jpg") => Some(include_bytes!("../assets/templates/ds/default.jpg")),
        ("dsm", "default.jpg") => Some(include_bytes!("../assets/templates/dsm/default.jpg")),
        ("dwight", "default.jpg") => Some(include_bytes!("../assets/templates/dwight/default.jpg")),
        ("elf", "default.jpg") => Some(include_bytes!("../assets/templates/elf/default.jpg")),
        ("elmo", "default.png") => Some(include_bytes!("../assets/templates/elmo/default.png")),
        ("ermg", "default.jpg") => Some(include_bytes!("../assets/templates/ermg/default.jpg")),
        ("exit", "default.png") => Some(include_bytes!("../assets/templates/exit/default.png")),
        ("fa", "default.jpg") => Some(include_bytes!("../assets/templates/fa/default.jpg")),
        ("facepalm", "default.jpg") => {
            Some(include_bytes!("../assets/templates/facepalm/default.jpg"))
        }
        ("fbf", "default.jpg") => Some(include_bytes!("../assets/templates/fbf/default.jpg")),
        ("feelsgood", "default.png") => {
            Some(include_bytes!("../assets/templates/feelsgood/default.png"))
        }
        ("fetch", "default.jpg") => Some(include_bytes!("../assets/templates/fetch/default.jpg")),
        ("fine", "default.png") => Some(include_bytes!("../assets/templates/fine/default.png")),
        ("fine", "default.gif") => Some(include_bytes!("../assets/templates/fine/default.gif")),
        ("firsttry", "default.png") => {
            Some(include_bytes!("../assets/templates/firsttry/default.png"))
        }
        ("fmr", "default.jpg") => Some(include_bytes!("../assets/templates/fmr/default.jpg")),
        ("friends", "default.png") => {
            Some(include_bytes!("../assets/templates/friends/default.png"))
        }
        ("fry", "default.png") => Some(include_bytes!("../assets/templates/fry/default.png")),
        ("fry", "default.gif") => Some(include_bytes!("../assets/templates/fry/default.gif")),
        ("fwp", "default.jpg") => Some(include_bytes!("../assets/templates/fwp/default.jpg")),
        ("gandalf", "default.jpg") => {
            Some(include_bytes!("../assets/templates/gandalf/default.jpg"))
        }
        ("gb", "default.jpg") => Some(include_bytes!("../assets/templates/gb/default.jpg")),
        ("gears", "default.jpg") => Some(include_bytes!("../assets/templates/gears/default.jpg")),
        ("genie", "default.png") => Some(include_bytes!("../assets/templates/genie/default.png")),
        ("ggg", "default.jpg") => Some(include_bytes!("../assets/templates/ggg/default.jpg")),
        ("glasses", "default.png") => {
            Some(include_bytes!("../assets/templates/glasses/default.png"))
        }
        ("gone", "default.jpg") => Some(include_bytes!("../assets/templates/gone/default.jpg")),
        ("grave", "default.png") => Some(include_bytes!("../assets/templates/grave/default.png")),
        ("gru", "default.jpg") => Some(include_bytes!("../assets/templates/gru/default.jpg")),
        ("grumpycat", "default.jpg") => {
            Some(include_bytes!("../assets/templates/grumpycat/default.jpg"))
        }
        ("hagrid", "default.jpg") => Some(include_bytes!("../assets/templates/hagrid/default.jpg")),
        ("happening", "default.jpg") => {
            Some(include_bytes!("../assets/templates/happening/default.jpg"))
        }
        ("happening", "default.gif") => {
            Some(include_bytes!("../assets/templates/happening/default.gif"))
        }
        ("harold", "default.jpg") => Some(include_bytes!("../assets/templates/harold/default.jpg")),
        ("headaches", "default.png") => {
            Some(include_bytes!("../assets/templates/headaches/default.png"))
        }
        ("hipster", "default.jpg") => {
            Some(include_bytes!("../assets/templates/hipster/default.jpg"))
        }
        ("home", "default.jpg") => Some(include_bytes!("../assets/templates/home/default.jpg")),
        ("icanhas", "default.jpg") => {
            Some(include_bytes!("../assets/templates/icanhas/default.jpg"))
        }
        ("imsorry", "default.jpg") => {
            Some(include_bytes!("../assets/templates/imsorry/default.jpg"))
        }
        ("inigo", "default.jpg") => Some(include_bytes!("../assets/templates/inigo/default.jpg")),
        ("interesting", "default.jpg") => Some(include_bytes!(
            "../assets/templates/interesting/default.jpg"
        )),
        ("ive", "default.png") => Some(include_bytes!("../assets/templates/ive/default.png")),
        ("iw", "default.png") => Some(include_bytes!("../assets/templates/iw/default.png")),
        ("jd", "default.jpg") => Some(include_bytes!("../assets/templates/jd/default.jpg")),
        ("jetpack", "default.jpg") => {
            Some(include_bytes!("../assets/templates/jetpack/default.jpg"))
        }
        ("jim", "default.png") => Some(include_bytes!("../assets/templates/jim/default.png")),
        ("joker", "default.jpg") => Some(include_bytes!("../assets/templates/joker/default.jpg")),
        ("jw", "default.png") => Some(include_bytes!("../assets/templates/jw/default.png")),
        ("jw", "alternate.png") => Some(include_bytes!("../assets/templates/jw/alternate.png")),
        ("keanu", "default.jpg") => Some(include_bytes!("../assets/templates/keanu/default.jpg")),
        ("kermit", "default.jpg") => Some(include_bytes!("../assets/templates/kermit/default.jpg")),
        ("khaby-lame", "default.jpg") => {
            Some(include_bytes!("../assets/templates/khaby-lame/default.jpg"))
        }
        ("kk", "luke.jpg") => Some(include_bytes!("../assets/templates/kk/luke.jpg")),
        ("kk", "default.jpg") => Some(include_bytes!("../assets/templates/kk/default.jpg")),
        ("kombucha", "default.png") => {
            Some(include_bytes!("../assets/templates/kombucha/default.png"))
        }
        ("kramer", "default.png") => Some(include_bytes!("../assets/templates/kramer/default.png")),
        ("kramer", "seinfeld.jpg") => {
            Some(include_bytes!("../assets/templates/kramer/seinfeld.jpg"))
        }
        ("leo", "default.jpg") => Some(include_bytes!("../assets/templates/leo/default.jpg")),
        ("light", "default.jpg") => Some(include_bytes!("../assets/templates/light/default.jpg")),
        ("live", "default.jpg") => Some(include_bytes!("../assets/templates/live/default.jpg")),
        ("live", "default.gif") => Some(include_bytes!("../assets/templates/live/default.gif")),
        ("ll", "default.jpg") => Some(include_bytes!("../assets/templates/ll/default.jpg")),
        ("lrv", "default.jpg") => Some(include_bytes!("../assets/templates/lrv/default.jpg")),
        ("made", "default.png") => Some(include_bytes!("../assets/templates/made/default.png")),
        ("mb", "box.jpg") => Some(include_bytes!("../assets/templates/mb/box.jpg")),
        ("mb", "default.jpg") => Some(include_bytes!("../assets/templates/mb/default.jpg")),
        ("mb", "default.gif") => Some(include_bytes!("../assets/templates/mb/default.gif")),
        ("mb", "flood.png") => Some(include_bytes!("../assets/templates/mb/flood.png")),
        ("michael-scott", "default.jpg") => Some(include_bytes!(
            "../assets/templates/michael-scott/default.jpg"
        )),
        ("midwit", "default.jpg") => Some(include_bytes!("../assets/templates/midwit/default.jpg")),
        ("millers", "default.png") => {
            Some(include_bytes!("../assets/templates/millers/default.png"))
        }
        ("mini-keanu", "default.jpg") => {
            Some(include_bytes!("../assets/templates/mini-keanu/default.jpg"))
        }
        ("mmm", "default.jpg") => Some(include_bytes!("../assets/templates/mmm/default.jpg")),
        ("money", "default.jpg") => Some(include_bytes!("../assets/templates/money/default.jpg")),
        ("money", "default.gif") => Some(include_bytes!("../assets/templates/money/default.gif")),
        ("mordor", "default.jpg") => Some(include_bytes!("../assets/templates/mordor/default.jpg")),
        ("morpheus", "default.jpg") => {
            Some(include_bytes!("../assets/templates/morpheus/default.jpg"))
        }
        ("mouth", "default.png") => Some(include_bytes!("../assets/templates/mouth/default.png")),
        ("mw", "default.jpg") => Some(include_bytes!("../assets/templates/mw/default.jpg")),
        ("nails", "default.png") => Some(include_bytes!("../assets/templates/nails/default.png")),
        ("nice", "default.jpg") => Some(include_bytes!("../assets/templates/nice/default.jpg")),
        ("noah", "default.jpg") => Some(include_bytes!("../assets/templates/noah/default.jpg")),
        ("noidea", "default.jpg") => Some(include_bytes!("../assets/templates/noidea/default.jpg")),
        ("ntot", "default.png") => Some(include_bytes!("../assets/templates/ntot/default.png")),
        ("oag", "default.jpg") => Some(include_bytes!("../assets/templates/oag/default.jpg")),
        ("officespace", "default.jpg") => Some(include_bytes!(
            "../assets/templates/officespace/default.jpg"
        )),
        ("older", "default.jpg") => Some(include_bytes!("../assets/templates/older/default.jpg")),
        ("oprah", "default.jpg") => Some(include_bytes!("../assets/templates/oprah/default.jpg")),
        ("oprah", "default.gif") => Some(include_bytes!("../assets/templates/oprah/default.gif")),
        ("panik-kalm-panik", "default.png") => Some(include_bytes!(
            "../assets/templates/panik-kalm-panik/default.png"
        )),
        ("patrick", "default.jpg") => {
            Some(include_bytes!("../assets/templates/patrick/default.jpg"))
        }
        ("patrick", "default.gif") => {
            Some(include_bytes!("../assets/templates/patrick/default.gif"))
        }
        ("perfection", "default.jpg") => {
            Some(include_bytes!("../assets/templates/perfection/default.jpg"))
        }
        ("persian", "default.jpg") => {
            Some(include_bytes!("../assets/templates/persian/default.jpg"))
        }
        ("philosoraptor", "default.jpg") => Some(include_bytes!(
            "../assets/templates/philosoraptor/default.jpg"
        )),
        ("pigeon", "default.jpg") => Some(include_bytes!("../assets/templates/pigeon/default.jpg")),
        ("pooh", "default.png") => Some(include_bytes!("../assets/templates/pooh/default.png")),
        ("pool", "default.png") => Some(include_bytes!("../assets/templates/pool/default.png")),
        ("prop3", "default.png") => Some(include_bytes!("../assets/templates/prop3/default.png")),
        ("ptj", "default.jpg") => Some(include_bytes!("../assets/templates/ptj/default.jpg")),
        ("puffin", "default.jpg") => Some(include_bytes!("../assets/templates/puffin/default.jpg")),
        ("red", "default.jpg") => Some(include_bytes!("../assets/templates/red/default.jpg")),
        ("regret", "default.jpg") => Some(include_bytes!("../assets/templates/regret/default.jpg")),
        ("remembers", "default.jpg") => {
            Some(include_bytes!("../assets/templates/remembers/default.jpg"))
        }
        ("reveal", "default.png") => Some(include_bytes!("../assets/templates/reveal/default.png")),
        ("right", "default.png") => Some(include_bytes!("../assets/templates/right/default.png")),
        ("rollsafe", "default.jpg") => {
            Some(include_bytes!("../assets/templates/rollsafe/default.jpg"))
        }
        ("rollsafe", "default.gif") => {
            Some(include_bytes!("../assets/templates/rollsafe/default.gif"))
        }
        ("sad-biden", "down.jpg") => Some(include_bytes!("../assets/templates/sad-biden/down.jpg")),
        ("sad-biden", "scowl.jpg") => {
            Some(include_bytes!("../assets/templates/sad-biden/scowl.jpg"))
        }
        ("sad-biden", "default.jpg") => {
            Some(include_bytes!("../assets/templates/sad-biden/default.jpg"))
        }
        ("sad-biden", "window.jpg") => {
            Some(include_bytes!("../assets/templates/sad-biden/window.jpg"))
        }
        ("sad-boehner", "default.jpg") => Some(include_bytes!(
            "../assets/templates/sad-boehner/default.jpg"
        )),
        ("sad-boehner", "really.jpg") => {
            Some(include_bytes!("../assets/templates/sad-boehner/really.jpg"))
        }
        ("sad-boehner", "sad.jpg") => {
            Some(include_bytes!("../assets/templates/sad-boehner/sad.jpg"))
        }
        ("sad-boehner", "frown.jpg") => {
            Some(include_bytes!("../assets/templates/sad-boehner/frown.jpg"))
        }
        ("sad-boehner", "what.jpg") => {
            Some(include_bytes!("../assets/templates/sad-boehner/what.jpg"))
        }
        ("sad-bush", "nervous.jpg") => {
            Some(include_bytes!("../assets/templates/sad-bush/nervous.jpg"))
        }
        ("sad-bush", "facebook.jpg") => {
            Some(include_bytes!("../assets/templates/sad-bush/facebook.jpg"))
        }
        ("sad-bush", "unsure.jpg") => {
            Some(include_bytes!("../assets/templates/sad-bush/unsure.jpg"))
        }
        ("sad-bush", "default.jpg") => {
            Some(include_bytes!("../assets/templates/sad-bush/default.jpg"))
        }
        ("sad-bush", "upset.jpg") => Some(include_bytes!("../assets/templates/sad-bush/upset.jpg")),
        ("sad-clinton", "nervous.jpg") => Some(include_bytes!(
            "../assets/templates/sad-clinton/nervous.jpg"
        )),
        ("sad-clinton", "default.jpg") => Some(include_bytes!(
            "../assets/templates/sad-clinton/default.jpg"
        )),
        ("sad-clinton", "ashamed.jpg") => Some(include_bytes!(
            "../assets/templates/sad-clinton/ashamed.jpg"
        )),
        ("sad-clinton", "sad.jpg") => {
            Some(include_bytes!("../assets/templates/sad-clinton/sad.jpg"))
        }
        ("sad-clinton", "frown.jpg") => {
            Some(include_bytes!("../assets/templates/sad-clinton/frown.jpg"))
        }
        ("sad-obama", "down.jpg") => Some(include_bytes!("../assets/templates/sad-obama/down.jpg")),
        ("sad-obama", "default.jpg") => {
            Some(include_bytes!("../assets/templates/sad-obama/default.jpg"))
        }
        ("sad-obama", "mad.jpg") => Some(include_bytes!("../assets/templates/sad-obama/mad.jpg")),
        ("sad-obama", "really.jpg") => {
            Some(include_bytes!("../assets/templates/sad-obama/really.jpg"))
        }
        ("sad-obama", "frown.jpg") => {
            Some(include_bytes!("../assets/templates/sad-obama/frown.jpg"))
        }
        ("sad-obama", "bow.jpg") => Some(include_bytes!("../assets/templates/sad-obama/bow.jpg")),
        ("sadfrog", "default.jpg") => {
            Some(include_bytes!("../assets/templates/sadfrog/default.jpg"))
        }
        ("saltbae", "default.jpg") => {
            Some(include_bytes!("../assets/templates/saltbae/default.jpg"))
        }
        ("same", "default.jpg") => Some(include_bytes!("../assets/templates/same/default.jpg")),
        ("sarcasticbear", "default.jpg") => Some(include_bytes!(
            "../assets/templates/sarcasticbear/default.jpg"
        )),
        ("say", "default.jpg") => Some(include_bytes!("../assets/templates/say/default.jpg")),
        ("sb", "default.jpg") => Some(include_bytes!("../assets/templates/sb/default.jpg")),
        ("scc", "default.jpg") => Some(include_bytes!("../assets/templates/scc/default.jpg")),
        ("seagull", "default.jpg") => {
            Some(include_bytes!("../assets/templates/seagull/default.jpg"))
        }
        ("sf", "default.jpg") => Some(include_bytes!("../assets/templates/sf/default.jpg")),
        ("sk", "default.jpg") => Some(include_bytes!("../assets/templates/sk/default.jpg")),
        ("ski", "default.png") => Some(include_bytes!("../assets/templates/ski/default.png")),
        ("slap", "default.png") => Some(include_bytes!("../assets/templates/slap/default.png")),
        ("snek", "default.png") => Some(include_bytes!("../assets/templates/snek/default.png")),
        ("soa", "default.jpg") => Some(include_bytes!("../assets/templates/soa/default.jpg")),
        ("sohappy", "default.jpg") => {
            Some(include_bytes!("../assets/templates/sohappy/default.jpg"))
        }
        ("sohot", "default.png") => Some(include_bytes!("../assets/templates/sohot/default.png")),
        ("soup-nazi", "default.jpg") => {
            Some(include_bytes!("../assets/templates/soup-nazi/default.jpg"))
        }
        ("sparta", "default.jpg") => Some(include_bytes!("../assets/templates/sparta/default.jpg")),
        ("spiderman", "default.jpg") => {
            Some(include_bytes!("../assets/templates/spiderman/default.jpg"))
        }
        ("spirit", "default.jpg") => Some(include_bytes!("../assets/templates/spirit/default.jpg")),
        ("spongebob", "default.jpg") => {
            Some(include_bytes!("../assets/templates/spongebob/default.jpg"))
        }
        ("ss", "default.jpg") => Some(include_bytes!("../assets/templates/ss/default.jpg")),
        ("stew", "default.jpg") => Some(include_bytes!("../assets/templates/stew/default.jpg")),
        ("stonks", "default.png") => Some(include_bytes!("../assets/templates/stonks/default.png")),
        ("stop", "default.jpg") => Some(include_bytes!("../assets/templates/stop/default.jpg")),
        ("stop-it", "default.jpg") => {
            Some(include_bytes!("../assets/templates/stop-it/default.jpg"))
        }
        ("success", "default.jpg") => {
            Some(include_bytes!("../assets/templates/success/default.jpg"))
        }
        ("tenguy", "default.jpg") => Some(include_bytes!("../assets/templates/tenguy/default.jpg")),
        ("toohigh", "default.jpg") => {
            Some(include_bytes!("../assets/templates/toohigh/default.jpg"))
        }
        ("touch", "default.jpg") => Some(include_bytes!("../assets/templates/touch/default.jpg")),
        ("tried", "default.jpg") => Some(include_bytes!("../assets/templates/tried/default.jpg")),
        ("trump", "default.jpg") => Some(include_bytes!("../assets/templates/trump/default.jpg")),
        ("ugandanknuck", "default.jpg") => Some(include_bytes!(
            "../assets/templates/ugandanknuck/default.jpg"
        )),
        ("vince", "default.jpg") => Some(include_bytes!("../assets/templates/vince/default.jpg")),
        ("wallet", "default.jpg") => Some(include_bytes!("../assets/templates/wallet/default.jpg")),
        ("waygd", "default.jpg") => Some(include_bytes!("../assets/templates/waygd/default.jpg")),
        ("waygd", "default.gif") => Some(include_bytes!("../assets/templates/waygd/default.gif")),
        ("wddth", "default.png") => Some(include_bytes!("../assets/templates/wddth/default.png")),
        ("whatyear", "default.jpg") => {
            Some(include_bytes!("../assets/templates/whatyear/default.jpg"))
        }
        ("winter", "default.jpg") => Some(include_bytes!("../assets/templates/winter/default.jpg")),
        ("wishes", "blank.png") => Some(include_bytes!("../assets/templates/wishes/blank.png")),
        ("wishes", "default.png") => Some(include_bytes!("../assets/templates/wishes/default.png")),
        ("wkh", "default.jpg") => Some(include_bytes!("../assets/templates/wkh/default.jpg")),
        ("woman-cat", "default.jpg") => {
            Some(include_bytes!("../assets/templates/woman-cat/default.jpg"))
        }
        ("wonka", "default.jpg") => Some(include_bytes!("../assets/templates/wonka/default.jpg")),
        ("worst", "default.jpg") => Some(include_bytes!("../assets/templates/worst/default.jpg")),
        ("xy", "default.jpg") => Some(include_bytes!("../assets/templates/xy/default.jpg")),
        ("yallgot", "default.jpg") => {
            Some(include_bytes!("../assets/templates/yallgot/default.jpg"))
        }
        ("yodawg", "default.jpg") => Some(include_bytes!("../assets/templates/yodawg/default.jpg")),
        ("yuno", "default.jpg") => Some(include_bytes!("../assets/templates/yuno/default.jpg")),
        ("zero-wing", "default.jpg") => {
            Some(include_bytes!("../assets/templates/zero-wing/default.jpg"))
        }
        _ => None,
    }
}
