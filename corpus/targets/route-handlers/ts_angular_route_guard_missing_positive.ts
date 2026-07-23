// [frensense]
// observation: Angular route definition lacks canActivate guard on a route that displays sensitive data
// impact: unauthenticated users can navigate to protected routes and access sensitive information
// improvement: add an AuthGuard implementing CanActivate that checks authentication state

import { NgModule } from '@angular/core'
import { RouterModule, type Routes } from '@angular/router'
import { DashboardComponent } from './dashboard.component'
import { AdminPanelComponent } from './admin-panel.component'

const routes: Routes = [
  { path: 'login', component: () => import('./login.component') },
  { path: 'dashboard', component: DashboardComponent },
  { path: 'admin', component: AdminPanelComponent }
]

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule {}
