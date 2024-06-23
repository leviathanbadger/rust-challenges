import { Routes } from "@angular/router";
import { SceneComponent } from "./scene/scene.component";
import { SpreadsheetComponent } from "./spreadsheet/spreadsheet.component";

export const primaryRoutes: Routes =  [{
    path: '',
    pathMatch: 'full',
    redirectTo: 'scene'
}, {
    path: 'scene',
    component: SceneComponent
}, {
    path: 'spreadsheet',
    component: SpreadsheetComponent
}];
