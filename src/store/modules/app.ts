import { defineStore } from "pinia";

export default defineStore("app", {
    state () {
        return {
            option: {} as IApp
        }
    },
    actions: {
        setOption (option: IApp) {
            this.option = option;
        },
        getOption () {
            return this.option;
        }
    }
});