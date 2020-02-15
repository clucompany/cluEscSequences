
pub trait EscResetAllAttr {}
pub trait EscResetAttr {}

crate::escseq_data! {
	@object(EscResetAllAttr) {
		pub Reset or ResetAllAttr				= "0";
	}
	@object(EscResetAttr) {
		pub ResetBold or ResetBright				= "21";
		pub ResetDim							= "22";
		pub ResetUnderline						= "24";
		pub ResetFlashing or ResetBlink			= "25";
		pub ResetReverse or ResetInvertedColors		= "27";
		pub ResetHidden or ResetInvisible			= "28";
		
		pub ResetForegColor						= "39";
		pub ResetBackColor						= "49";
	}
}
