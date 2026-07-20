// SAFE: AuthGuard added to all protected routes via canActivate

import { Injectable, NgModule } from '@angular/core'
import { RouterModule, type Routes, type CanActivate, type Router } from '@angular/router'
import { DashboardComponent } from './dashboard.component'
import { AdminPanelComponent } from './admin-panel.component'

@Injectable({ providedIn: 'root' })
export class AuthGuard implements CanActivate {
  constructor(private router: Router) {}

  canActivate(): boolean {
    const token = localStorage.getItem('auth_token')
    if (!token) {
      this.router.navigate(['/login'])
      return false
    }
    return true
  }
}

const routes: Routes = [
  { path: 'login', component: () => import('./login.component') },
  { path: 'dashboard', component: DashboardComponent, canActivate: [AuthGuard] },
  { path: 'admin', component: AdminPanelComponent, canActivate: [AuthGuard] }
]

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule {}
