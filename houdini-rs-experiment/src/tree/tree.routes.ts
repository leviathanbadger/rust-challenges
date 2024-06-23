import { Routes } from "@angular/router";
import { TreeComponent } from "./tree/tree.component";

export const treeRoutes: Routes =  [{
    path: '',
    pathMatch: 'full',
    redirectTo: 'tree'
}, {
    path: 'tree',
    component: TreeComponent
}];
