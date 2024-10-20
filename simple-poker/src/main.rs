use rand::seq::SliceRandom;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}
#[derive(Debug, Copy, Clone, PartialEq)]
struct Card {
    suit: Suit,
    rank: i32,
}

fn main() {
    let mut deck: Vec<Card> = Vec::new();
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    for suit in suits {
        for rank in 1..=13 {
            deck.push(Card { suit, rank });
        }
    }
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);

    let mut hand: Vec<Card> = Vec::new();
    for _ in 0..5 {
        hand.push(deck.pop().unwrap());
    }


    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    println!("----------------");
    for (i, card) in hand.iter().enumerate() {
        println!("{}: {:?} {}", i+1, card.suit, card.rank);
    }
    
    println!("----------------");

    println!("手札を交換しますか？");
    println!("交換する場合は交換するカードの番号をカンマ区切りで入力してください");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<usize> = input.split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<usize>>();

    for number in numbers {
        hand[number-1] = deck.pop().unwrap();
    }

    hand.sort_by(|a, b| a.rank.cmp(&b.rank));
    println!("----------------");
    for (i, card) in hand.iter().enumerate() {
        println!("{}: {:?} {}", i+1, card.suit, card.rank);
    }

    //フラッシュの判定
    let suit = hand.first().unwrap().suit;
    let falsh = hand.iter().all(|card| card.suit == suit);

    //ペアの数の判定
    let mut count = 0;
    for i in 0..hand.len() - 1{
        for j in i + 1..hand.len(){
            if hand[i].rank == hand[j].rank {
                count += 1;
            }
        }
    }

    if falsh && count == 4 {
        println!("フルハウスです");
    } else if falsh {
        println!("フラッシュです");
    } else if count == 4 {
        println!("ワンペアです");
    } else if count == 6 {
        println!("スリーカードです");
    } else {
        println!("ノーペアです");
    }
}
