use crate::State;
pub fn pattern1(state: &mut State) {
    // cross alternation
    state.lines[21][50] = true;
    state.lines[22][50] = true;
    state.lines[23][50] = true;
}
pub fn pattern2(state: &mut State) {
    // still life
    state.lines[19][51] = true;
    state.lines[20][51] = true;
    state.lines[19][50] = true;
    state.lines[21][50] = true;
    state.lines[22][49] = true;
    state.lines[23][48] = true;
    state.lines[22][47] = true;
    state.lines[23][47] = true;
}
pub fn pattern3(state: &mut State) {
    // cannon into stable cross
    state.lines[25][50] = true;
    state.lines[25][51] = true;
    state.lines[25][52] = true;
    state.lines[26][49] = true;
    state.lines[26][53] = true;
    state.lines[27][48] = true;
    state.lines[27][54] = true;
    state.lines[28][48] = true;
    state.lines[28][54] = true;
    state.lines[29][51] = true;
    state.lines[30][49] = true;
    state.lines[30][53] = true;
    state.lines[31][50] = true;
    state.lines[31][52] = true;
}
pub fn pattern4(state: &mut State) {
    // unpredictible
    state.lines[19][51] = true;
    state.lines[20][50] = true;
    state.lines[20][51] = true;
    state.lines[20][52] = true;
    state.lines[21][50] = true;
}
pub fn pattern5(state: &mut State) {
    // cannon
    state.lines[5][30] = true;
    state.lines[5][31] = true;
    state.lines[6][30] = true;
    state.lines[6][31] = true;
    state.lines[15][30] = true;
    state.lines[15][31] = true;
    state.lines[15][32] = true;
    state.lines[16][29] = true;
    state.lines[16][33] = true;
    state.lines[17][28] = true;
    state.lines[17][34] = true;
    state.lines[18][28] = true;
    state.lines[18][34] = true;
    state.lines[19][31] = true;
    state.lines[20][29] = true;
    state.lines[20][33] = true;
    state.lines[21][30] = true;
    state.lines[21][31] = true;
    state.lines[21][32] = true;
    state.lines[22][31] = true;
    state.lines[25][28] = true;
    state.lines[25][29] = true;
    state.lines[25][30] = true;
    state.lines[26][28] = true;
    state.lines[26][29] = true;
    state.lines[26][30] = true;
    state.lines[27][27] = true;
    state.lines[27][31] = true;
    state.lines[29][26] = true;
    state.lines[29][27] = true;
    state.lines[29][31] = true;
    state.lines[29][32] = true;
    state.lines[39][28] = true;
    state.lines[39][29] = true;
    state.lines[40][28] = true;
    state.lines[40][29] = true;
}
