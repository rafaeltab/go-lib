use sgf_parse::{go::Prop, parse};

use crate::go::{
    board::FlexibleBoard,
    coordinate::FlexibleCoordinate,
    game::Game,
    player::Player,
    playermove::{Move, PlaceStoneMove},
};

pub struct ParsedGame {
    pub width: u16,
    pub height: u16,
    pub moves: Vec<Move>,
}

impl ParsedGame {
    pub fn run<TBoardFactory: Fn((u16, u16)) -> TBoard, TBoard: FlexibleBoard>(
        &self,
        board_factory: TBoardFactory,
    ) -> Game<TBoard> {
        let board = board_factory((self.width, self.height));
        let mut game = Game::new(board);

        for m in &self.moves {
            game.make_move(m).expect("SGF had invalid move!");
        }

        game
    }
}

pub fn parse_sgf(sgf: &str) -> ParsedGame {
    let mut moves = vec![];
    let mut width: u16 = 0;
    let mut height: u16 = 0;

    let collection = parse(sgf).expect("Invalid sgf file");
    let go_game = collection.first().unwrap().as_go_node().unwrap();
    let size = go_game.get_property("SZ").expect("Game had no size");
    if let Prop::SZ((w, h)) = size {
        println!("width: {}, height: {}", w, h);
        width = *w as u16;
        height = *h as u16;
    }

    for node in go_game.main_variation() {
        if let Some(prop) = node.get_move() {
            if let Prop::B(m) = prop {
                moves.push(match m {
                    sgf_parse::go::Move::Pass => Move::Skip {
                        player: Player::Black,
                    },
                    sgf_parse::go::Move::Move(point) => Move::PlaceStone(PlaceStoneMove {
                        player: Player::Black,
                        coord: FlexibleCoordinate {
                            x: point.x as u16,
                            y: point.y as u16,
                        },
                    }),
                });
            }

            if let Prop::W(m) = prop {
                moves.push(match m {
                    sgf_parse::go::Move::Pass => Move::Skip {
                        player: Player::White,
                    },
                    sgf_parse::go::Move::Move(point) => Move::PlaceStone(PlaceStoneMove {
                        player: Player::White,
                        coord: FlexibleCoordinate {
                            x: point.x as u16,
                            y: point.y as u16,
                        },
                    }),
                });
            }
        }
    }

    ParsedGame {
        width,
        height,
        moves,
    }
}

#[cfg(test)]
mod test {
    use crate::{
        go::{bitmask::TestMask, bitmask_board::BitMaskBoard, bitmask19::BitMask19},
        parser::gsf::parse_sgf,
    };

    #[test]
    fn should_parse() {
        let input = "(;FF[4]
CA[UTF-8]
GM[1]
DT[2026-01-14]
PC[OGS: https://online-go.com/game/83164571]
GN[Friendly Match]
PB[ikbenrafaelbieze2001]
PW[lululouisjin]
BR[31k]
WR[3k]
TM[300]OT[3x30 byo-yomi]
RE[W+238.5]
SZ[19]
KM[6.5]
RU[Japanese]
;B[dd]
(;W[pd]
C[ikbenrafaelbieze2001: time to get fucked by a 3kyu lets go
]
(;B[dp]
(;W[pq]
(;B[qc]
(;W[qd]
(;B[pc]
(;W[od]
(;B[oc]
(;W[nc]
(;B[nb]
(;W[mc]
(;B[mb]
(;W[lc]
(;B[lb]
(;W[pk]
(;B[rd]
(;W[re]
(;B[rc]
(;W[kc]
(;B[kb]
(;W[jc]
(;B[hd]
(;W[qf]
(;B[jp]
(;W[fd]
(;B[gc]
(;W[fc]
(;B[hb]
(;W[fb]
(;B[db]
(;W[jb]
(;B[ja]
(;W[ia]
(;B[ka]
(;W[ib]
(;B[ge]
(;W[fe]
(;B[gf]
(;W[ff]
(;B[df]
(;W[gg]
(;B[eg]
(;W[hg]
(;B[ie]
(;W[if]
(;B[je]
(;W[kf]
(;B[jf]
(;W[jg]
(;B[ke]
(;W[lf]
(;B[hf]
(;W[ig]
(;B[gd]
(;W[fg]
(;B[ic]
(;W[gb]
(;B[jd]
(;W[ha]
(;B[hc]
(;W[ch]
(;B[dh]
(;W[cc]
(;B[ce]
(;W[dc]
(;B[ec]
(;W[ed]
(;B[cd]
(;W[bc]
(;B[bd]
(;W[cg]
(;B[bf]
(;W[di]
(;B[eh]
(;W[cj]
(;B[fi]
(;W[fj]
(;B[gi]
(;W[gj]
(;B[el]
(;W[hi]
(;B[hk]
(;W[hj]
(;B[jj]
(;W[ik]
(;B[ij]
(;W[jk]
(;B[kj]
(;W[kk]
(;B[li]
(;W[lk]
(;B[le]
(;W[me]
(;B[mf]
(;W[mg]
(;B[nf]
(;W[ne]
(;B[ng]
(;W[mh]
(;B[nh]
(;W[mi]
(;B[mj]
(;W[ni]
(;B[oh]
(;W[oi]
(;B[pi]
(;W[pj]
(;B[ph]
(;W[nk]
(;B[nj]
(;W[oj]
(;B[se]
(;W[sf]
(;B[sd]
(;W[cl]
(;B[dm]
(;W[bn]
(;B[cn]
(;W[bo]
(;B[cq]
(;W[gp]
(;B[ho]
(;W[go]
(;B[hp]
(;W[gm]
(;B[gl]
(;W[hn]
(;B[il]
(;W[fm]
(;B[fl]
(;W[jn]
(;B[in]
(;W[im]
(;B[io]
(;W[jl]
(;B[hl]
(;W[hm]
(;B[ko]
(;W[bp]
(;B[bq]
(;W[do]
(;B[co]
(;W[cp]
(;B[eq]
(;W[eo]
(;B[fq]
(;W[gq]
(;B[gr]
(;W[hr]
(;B[fr]
(;W[hq]
(;B[cm]
(;W[bm]
(;B[lg]
(;W[kg]
(;B[lh]
(;W[lp]
(;B[lj]
(;W[mk]
(;B[hh]
(;W[gh]
(;B[ih]
(;W[dr]
(;B[dq]
(;W[aq]
(;B[ar]
(;W[ap]
(;B[cr]
(;W[qo]
(;B[qj]
(;W[qk]
(;B[rj]
(;W[rh]
(;B[sg]
(;W[rf]
(;B[rg]
(;W[qg]
(;B[of]
(;W[fp]
(;B[oe]
(;W[rk]
(;B[pe]
(;W[qe]
(;B[mn]
(;W[lo]
(;B[ln]
(;W[kn]
(;B[np]
(;W[kq]
(;B[jq]
(;W[mq]
(;B[nq]
(;W[jr]
C[ikbenrafaelbieze2001: I feel like everything I tried is just blocked with a massive concrete wall
]
(;B[no]
(;W[on]
(;B[nr]
(;W[mr]
(;B[kp]
(;W[kr]
(;B[ir]
(;W[iq]
(;B[mp]
(;W[gs]
(;B[fs]
(;W[hs]
(;B[ms]
(;W[ac]
(;B[ks]
(;W[lq]
(;B[js]
(;W[is]
(;B[ns]
C[ikbenrafaelbieze2001: but at the same time I don't feel stupid
]
(;W[ep]
(;B[er]
(;W[]
(;B[]
C[ikbenrafaelbieze2001: hahaha I loved this
ikbenrafaelbieze2001: thank you for not exiting the game <3
]
)))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))";

        let game = parse_sgf(input);

        for _ in 0..1000 {
            let res = game.run(|size| BitMaskBoard::new(BitMask19::init));
        }
        // println!("{}", res.get_board().display());
        // panic!();
    }
}
