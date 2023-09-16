using System;
using System.Linq;
using System.Web;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;

namespace nettrash.Controllers
{
	[ApiController]
	[Route("[controller]")]
	public class TextController : ControllerBase
	{
		#region Private properties



		private readonly ILogger<TextController> _logger;



		#endregion
		#region Public constructors



		public TextController(ILogger<TextController> logger)
		{
			_logger = logger;
		}



		#endregion
		#region Public methods



		[HttpPost("base64")]
		public Response.Base64 Base64(Request.Base64 request)
		{
			try
			{
				string sResult = "";
				if (request.Encode)
				{
					sResult = Convert.ToBase64String(System.Text.Encoding.UTF8.GetBytes(request.Source));
				}
				else
				{
					byte[] result = Convert.FromBase64String(request.Source);
					sResult = System.Text.Encoding.UTF8.GetString(result);
				}
				return new Response.Base64 { Result = true, Value = sResult };
			}
			catch (Exception ex)
			{
				return new Response.Base64 { Result = false, Value = ex.Message };
			}
		}

		[HttpPost("url")]
		public Response.Url Uri(Request.Url request)
		{
			try
			{
				string sResult = "";
				if (request.Encode)
				{
					sResult = HttpUtility.UrlEncode(request.Source);
				}
				else
				{
					sResult = HttpUtility.UrlDecode(request.Source);
				}
				return new Response.Url { Result = true, Value = sResult };
			}
			catch (Exception ex)
			{
				return new Response.Url { Result = false, Value = ex.Message };
			}
		}

		[HttpPost("hex")]
		public Response.Hex Hex(Request.Hex request)
		{
			try
			{
				string sResult = "";
				if (request.Encode)
				{
					sResult = BitConverter.ToString(System.Text.Encoding.UTF8.GetBytes(request.Source)).Replace("-", string.Empty).ToUpper();
				}
				else
				{
					string hex = request.Source.Replace(" ", string.Empty).Replace("-", string.Empty); //BitConverter remove
					sResult = System.Text.Encoding.UTF8.GetString(Enumerable.Range(0, hex.Length)
							.Where(x => x % 2 == 0)
							.Select(x => Convert.ToByte(hex.Substring(x, 2), 16))
							.ToArray());
				}
				return new Response.Hex { Result = true, Value = sResult };
			}
			catch (Exception ex)
			{
				return new Response.Hex { Result = false, Value = ex.Message };
			}
		}

		[HttpPost("regex")]
		public Response.Regex Regex(Request.Regex request)
		{
			try
			{
				bool bMatched = System.Text.RegularExpressions.Regex.IsMatch(request.Text, request.Source);
				return new Response.Regex
				{
					Result = bMatched,
					Value = $"Is Matched: {bMatched}.\nMatches:\n{string.Join("\n", System.Text.RegularExpressions.Regex.Matches(request.Text, request.Source).Select(m => m.Value).ToArray())}."
				};
			}
			catch (Exception ex)
			{
				return new Response.Regex { Result = false, Value = ex.Message };
			}
		}



		#endregion
	}
}