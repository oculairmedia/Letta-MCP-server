def process_text(text: str, mode: str = "upper") -> str:
    """
    Process text according to the specified mode.
    
    This function transforms text based on the mode parameter,
    supporting various text transformations.
    
    Args:
        text: The input text to process
        mode: The processing mode - either 'upper', 'lower', or 'title'
        
    Returns:
        str: The processed text according to the specified mode
    """
    if mode == "upper":
        return text.upper()
    elif mode == "lower":
        return text.lower()
    elif mode == "title":
        return text.title()
    else:
        return text