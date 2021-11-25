"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.mochaHooks = void 0;
const redspot_1 = require("redspot");
const { api } = redspot_1.network;
exports.mochaHooks = {
    afterAll: () => {
        api.disconnect();
    }
};
