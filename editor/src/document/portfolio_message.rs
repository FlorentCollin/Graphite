use super::clipboards::Clipboard;
use crate::message_prelude::*;

use graphene::LayerId;

use serde::{Deserialize, Serialize};

/// The portfolio is the collection of open documents in the Graphite editor. These messages pertain to the handling of multiple documents and to systems that are shared across documents.
#[remain::sorted]
#[impl_message(Message, Portfolio)]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum PortfolioMessage {
	// Sub-messages
	#[remain::unsorted]
	#[child]
	Document(DocumentMessage),

	// Messages
	AutoSaveActiveDocument,
	AutoSaveDocument {
		document_id: u64,
	},
	CloseActiveDocumentWithConfirmation,
	CloseAllDocuments,
	CloseAllDocumentsWithConfirmation,
	CloseDocument {
		document_id: u64,
	},
	CloseDocumentWithConfirmation {
		document_id: u64,
	},
	Copy {
		clipboard: Clipboard,
	},
	Cut {
		clipboard: Clipboard,
	},
	NewDocument,
	NextDocument,
	OpenDocument,
	OpenDocumentFile {
		document_name: String,
		document_serialized_content: String,
	},
	OpenDocumentFileWithId {
		document_id: u64,
		document_name: String,
		document_is_saved: bool,
		document_serialized_content: String,
	},
	Paste {
		clipboard: Clipboard,
	},
	PasteIntoFolder {
		clipboard: Clipboard,
		folder_path: Vec<LayerId>,
		insert_index: isize,
	},
	PrevDocument,
	RequestAboutGraphiteDialog,
	RequestNewFileDialog,
	SelectDocument {
		document_id: u64,
	},
	UpdateOpenDocumentsList,
}
