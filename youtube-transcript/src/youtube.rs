use crate::config::Config;
use crate::parser::{HTMLParser, TranscriptParser};
use reqwest::Client;
use roxmltree::Document;
use std::error::Error;
pub struct Youtube<'a, 'b> {
    link: &'a str,
    config: &'b Config,
}

impl<'a, 'b> Youtube<'a, 'b> {
    pub fn link(link: &'a str, config: &'b Config) -> Self {
        Self { link, config }
    }
}

impl<'a, 'b> Youtube<'a, 'b> {
    pub async fn get_transcript(&self) -> Result<String, Box<dyn Error>> {
        let client = Client::default();
        let response = client.get(self.link).send().await?;
        let c = response
            .text()
            .await?
            .caption(self.config.parser.from, self.config.parser.to)?;
        let response = client.get(c.base_url).send().await?;
        let trans_resp = response.text().await?;
        let doc = Document::parse(&trans_resp)?;
        let t = TranscriptParser::parse(&doc)?;
        Ok(t.describe())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Config;
    #[tokio::test]
    #[ignore = "Requires mocking youtube response"]
    async fn test_u_trans() {
        let config = Config::default();
        let u = Youtube::link("https://www.youtube.com/watch?v=GJLlxj_dtq8", &config);
        let transcript = u.get_transcript().await.unwrap();
        assert_eq!(
            transcript,
            "Hey, how&#39;s it going Dave 2d here?This is a Microsoft Surface go and when they first announced it I was interested in itIt seemed like a pretty compelling device having used it for a little whileI really think this is seriously the best product that Microsoft has put out in a very long time this thing starts at $400I don&#39;t think that base configuration is where you want to spend the money though. They have a mid tier one550 quite a bit more but you&#39;re getting double the RAM double the storage but significantly faster storageThat is the model that I think most people should pick up if you can afford that price bumpso this unit here, is that mid tier model the$550 unit and IReally like it. Ok, let&#39;s go around. This thing build quality is great. It&#39;s a surface productIt has a magnesium enclosure fit and finish on this is really well donetheTop surface has these new rounded edges and it actually makes the device a lot more comfortable to holdNot that the original surface products are like uncomfortableBut this small detail just makes it that much more ergonomic and that much more inviting to useIt&#39;s a nice touch and I think Microsoft should put this kind ofRounded edge on all of their products because it does make a difference. The screen is a 10 inch screenI thought I&#39;d be a little bit small for what I doBut it actually isn&#39;t it is noticeably smaller compared to like a 12 or 13 inch screen, but it doesn&#39;t feel particularly crampedIt&#39;s still a very usable surface area the bezels around that screen though are thick now visuallyIt&#39;s not attractive right having thick bezels. Like this doesn&#39;t look goodBut when you&#39;re actually using it, you won&#39;t notice it you&#39;ll be focused on your contentit&#39;s just that when this devices off or it&#39;s just sitting there and you&#39;re kind ofexamining it visually the bezels are thick the panel itself is nice its sharp great colors and brightness andHitting a price point like this with this kind of screenIt could not have been easy. Like we see four or five hundred dollar devices out there that have terrible screensThis thing looks really good. There is pen support as usual and feels relatively lag free to meI&#39;m not an artistBut the surface area feels reasonably sized for people that want to use it for any kind of digital creative workNow on the side are two speakers and they sound really good for this kind of device sizenice body to the soundExcellent stereo separation just from the positioning and you just get really clean audio that gets to a decent volumeYou also get a killer killer webcam $400 gets your webcam of this qualityit&#39;s actually one of the best kans I&#39;ve seen on any laptop period but when you compareThis webcam to something like a 12-inch MacBook. It just blows my mindI mean if you can stick a webcam like this into a $400 device, there&#39;s no excuse for other peopleThey should be using really good webcams and no one else is doing it, but surface does so good for themthis device though is not complete without the keyboard and the keyboard is aHundred bucks, which is crazy expensive you think about it. That&#39;s like at the base model. That&#39;s 20% of the cost, butThat&#39;s what we have. Okay, when it&#39;s connected up and it connects magnetically. It is an awesome. Awesomeproductivity deviceSo I was concerned that this keyboard would be really small and cramped and just kind of weird feeling because it is a lot smallerThan the regular service devices. It&#39;s not cramped. It&#39;s excellent. It does take a little bit of time to get used to itBut it is a really comfortable keyboard the trackpad feels good. It&#39;s a surface productSo tracking is accurate and gestures work nicely. But the pad is a little small. Maybe it&#39;s a visual thingI just wish there&#39;s a little bit more surface area to this trackpad. Okay performance on this device isGood, it&#39;s not amazing. It&#39;s a Pentium Gold chip and most productivity stuffLike emails web browsing or any kind of work-related stuff runs really smoothly on thisSo the drive feeds on the mid-tier model actually really good fast read speed but on the slower drive of the base model the wholeSystem is gonna feel a bit more sluggish and that reason alone makes it worth it to upgrade to the mid-tier modelBattery life is also pretty good getting around seven hours of battery life and to charge itYou can either use the included surface connect adapter or you can use the USB C portI really wish that the included adapter was USB C but its surface connect because well, that&#39;s Microsoft&#39;s for youOkay gaming performance. I was actually surprised by thisYou&#39;re not gonna be able to play some killer triple-a titlesBut light games are pretty good on this thing if you want to pick it up for some casual light gamesIt&#39;ll do the trick nowThe surface go is still a surface product through and through so if they&#39;re issues you had with surface products in the pastYou may have those same issues with this one. Like if you need more ports, there&#39;s still only one portIt&#39;s use BC this yearBut it&#39;s still only one port if you don&#39;t like the kickstand on your lapLike it&#39;s not an ideal situation for lap use, but it does work reasonablyWell plus infinite positions up to a certain degree, but I don&#39;t knowThis makes it fairly usable for most people I think but if you&#39;ve had issues in the past same issues nowOverall though really good product. I think for studentsThis is such a good option you get so much versatility on this thingYou get a great keyboard for taking notesYou can pull up course material and stuff in classGood option great for me to consumption for like a secondary device if you want it for thatI think you can&#39;t go wrong with this it is however notCheap once you add everything up together like the keyboard and like the mid tier unitIt&#39;s not the $400 device that they&#39;re kind of marketingSo you kind of have to take that into consideration, but overall I like this thing. Ok. Hope you guys enjoyed this video thumbsWe liked it subs we loved it. See you guys next time"
        );
    }
}
