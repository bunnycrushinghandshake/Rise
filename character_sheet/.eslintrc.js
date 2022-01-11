module.exports = {
  env: {},
  globals: {
    WeakMap: true,
    Set: true,
    setAttrs: true,
    setInterval: true,
    clearInterval: true,
    clearTimeout: true,
    setTimeout: true,
    playerIsGM: true,
    getObj: true,
    findObjs: true,
    filterObjs: true,
    createObj: true,
    sendChat: true,
    log: true,
    toFront: true,
    toBack: true,
    randomInteger: true,
    setDefaultTokenForCharacter: true,
    spawnFx: true,
    spawnFxBetweenPoints: true,
    spawnFxWithDefinition: true,
    playJukeboxPlaylist: true,
    stopJukeboxPlaylist: true,
    sendPing: true,
    state: true,
    globalconfig: true,
    _: true,
    Campaign: true,
    getAllObjs: true,
    getAttrByName: true,
    getAttrs: true,
    getSectionIDs: true,
    onSheetWorkerCompleted: true,
    on: true,
    Promise: true,
    Uint32Array: true,
    takeCardFromPlayer: true,
    giveCardToPlayer: true,
    recallCards: true,
    shuffleDeck: true,
    drawCard: true,
    cardInfo: true,
    playCardToTable: true,
  },
  extends: "eslint:recommended",
  parserOptions: {
    ecmaVersion: 2017,
    ecmaFeatures: {
      impliedStrict: true,
      experimentalObjectRestSpread: true,
    },
    sourceType: "module",
  },
  plugins: [],
  rules: {
    "no-console": "warn",
    "linebreak-style": ["error", "unix"],
    semi: ["error", "always"],
  },
};