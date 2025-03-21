import SwiftUI
import SwiftWorkspace

struct OutlineSection: View {
        
    var root: File

    var body: some View {
        let rootOutlineBranch = OutlineBranch(file: root, level: -1)
        ScrollView {
            VStack(alignment: .leading, spacing: 2) {
                // The padding in the section header is there to adjust for the inset hack.
                rootOutlineBranch
                Spacer()
            }
            .listStyle(SidebarListStyle())
            .frame(minWidth: 10, maxWidth: .infinity, maxHeight: .infinity)
            .padding()
            // A hack for list row insets not working. This hack also applies to the section header though.
        }.contextMenu {
            OutlineContextMenu (meta: root, branchState: nil)
        }
        .refreshable {
            DI.workspace.requestSync()
        }
    }
}
