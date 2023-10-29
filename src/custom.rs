use std::fmt::Debug;

use dyn_clone::DynClone;

use crate::{Element, Flavor, Serialize};

pub trait CustomElement: Debug + DynClone + Send {
	/// Serialize this element into a string of XML.
	///
	/// See [`crate::util`] for serialization utilities.
	fn serialize_to_string(&self, flavor: Flavor) -> anyhow::Result<String>;

	fn tag_name(&self) -> Option<&str> {
		None
	}

	fn children(&self) -> Option<&Vec<Element>> {
		None
	}
}

dyn_clone::clone_trait_object!(CustomElement);

impl Serialize for Box<dyn CustomElement> {
	fn serialize<W: std::io::Write>(&self, writer: &mut W, flavor: Flavor) -> anyhow::Result<()> {
		writer.write_all(CustomElement::serialize_to_string(self.as_ref(), flavor)?.as_bytes())?;
		Ok(())
	}

	fn serialize_to_string(&self, flavor: Flavor) -> anyhow::Result<String> {
		CustomElement::serialize_to_string(self.as_ref(), flavor)
	}
}
