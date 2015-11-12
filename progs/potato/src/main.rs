//Sources for potato facts:
// http://idahopotatomuseum.com/potato-facts/
// http://www.potatogoodness.com/all-about-potatoes/potato-fun-facts-history/
// http://farmfreshdirect.net/our-crops/potato-facts/

use std::env;

fn main() {
	//Arguments
    let args: Vec<String> = env::args().collect();

    //List of potato facts
    let facts[String; 22] =    ["Today potatoes are grown in all 50 states of the USA 	and in about 125 countries throughout the world.",
    							"The sweet potato belongs in the same family as morning glories while the white potato belongs to the same group as tomatoes, tobacco, chile pepper, eggplant and the petunia",
    							"The potato is about 80% water and 20% solids.",
    							"An 8 ounce baked or boiled potato has only about 100 calories.",
    							"The average American eats about 124 pounds of potatoes per year while Germans eat about twice as much.",
    							"In 1974, an Englishman named Eric Jenkins grew 370 pounds of potatoes from one plant.",
    							"Thomas Jefferson gets the credit for introducing “french fries” to America when he served them at a White House dinner.",
    							"According to the Guinness Book of World Records, the largest potato grown was 7 pounds 1 ounce by J. East (1953) and J. Busby (1982) of Great Britain.",
    							"The world’s largest potato chip crisp (on exhibit at the Potato Museum) was produced by the Pringle’s Company in Jackson, TN, in 1990. It measures 23\″ x 14.5\″",
    							"In October 1995, the potato became the first vegetable to be grown in space. NASA and the University of Wisconsin, Madison, created the technology with the goal of feeding astronauts on long space voyages, and eventually, feeding future space colonies.",
    							"Potato blossoms used to be a big hit in royal fashion. Potatoes first became fashionable when Marie Antoinette paraded through the French countryside wearing potato blossoms in her hair.",
    							"During the Alaskan Klondike gold rush, (1897-1898) potatoes were practically worth their weight in gold. Potatoes were valued for their vitamin C.  And gold, at that time, was more plentiful than nutritious foods!",
    							"In October 1995, the potato became the first vegetable to be grown in space. NASA and the University of Wisconsin, Madison, created the technology with the goal of feeding astronauts on long space voyages, and eventually, feeding future space colonies.",
    							"Potatoes were the first vegetable grown in space",
    							"Potatoes are the best-selling side dish in American restaurants",
    							"Potatoes have one of the first commodity groups to develop and use an FDA-approved nutrition label",
    							"Only 19% of Americans rate potatoes as being “great” for gluten-free. Seriously? Potatoes are TOTALLY gluten-free.",
    							"Less than 3% of Americans meet the FDA guidelines for potassium intake. Quick, eat a potato!",
    							"Research suggests that diets rich in potassium and low in sodium reduce the risk of hypertension and stroke. Helloooooo potato!",
    							"Research also indicates that diets high in potassium-rich fruits and vegetables may help maintain lean body mass and bone",
    							"In addition to vitamins and minerals, potatoes also have an assortment of phytochemicals with antioxidant potential, most notably carotenoids and anthocyanins",
    							"Potatoes have shown to help keep you feeling full, longer. Great!"];
 
    
    println!("Here are {:?} potato facts!:, &args[1]);
   
}
