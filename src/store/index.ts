import useLogStore from './modules/log';

export default function useStore() {
  return {
    log: useLogStore(),
  }
}