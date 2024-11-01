interface IApp {
    databaseUrl:string;
    currentPage: string;
    vmExePath: string;
    masterMacPath: string;
    sonMacPath: string;
    appleIDs: any[];
    serialNumbers: any[];
    vms: any[];
    runNumbers: number;
    maxRunNumbers: string;
    isCloning: boolean;
}