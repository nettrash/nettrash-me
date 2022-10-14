namespace nettrash.Controllers.Response
{
	public class Luhn
	{
		#region Public properties



		public bool Result { get; set; }

		public bool LuhnResult { get; set; }

		public string ErrorText { get; set; }



		#endregion
		#region Public constructors



		public Luhn()
		{
		}



		#endregion
	}
}