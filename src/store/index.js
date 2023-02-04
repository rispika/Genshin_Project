import { createStore } from 'vuex'
import { invoke } from '@tauri-apps/api';

/**
 * 创建仓库和导出
 */
export default createStore({
    state: {
        theme: "light",
        flagMenu: true,
        flagLoading: false,
        loadingMargin: 0,
        loadingMsg: '请稍等一会!',
        UID: 'NONE',
        noticeInitFlag: false
    },
    actions: {
        async updateUID() {
            console.log(`开始尝试获取uid`);
            const uids = await invoke('get_data_uid')
            console.log(`当前uid为${uids}`);
            if (Object.keys(uids).length !== 0) {
                console.log(`存在uids`);
                this.commit('setUID',uids[0])
            }
        }
    },
    mutations: {
        setTheme(state, newTheme) {
            state.theme = newTheme
        },
        setLoadingMargin(state, newMargin) {
            state.loadingMargin = newMargin
        },
        setLoadingMsg(state, newMsg) {
            state.loadingMsg = newMsg
        },
        setUID(state, newUID) {
            state.UID = newUID
        },
        setFlagLoading(state, newFlag) {
            state.flagLoading = newFlag
        },
        setNoticeInitFlag(state, newFlag) {
            state.noticeInitFlag = newFlag
        },
        setFlagMenu(state, newFlag) {
            state.flagMenu = newFlag
        }
    }
})