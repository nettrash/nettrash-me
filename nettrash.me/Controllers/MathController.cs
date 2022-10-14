using System;
using System.Linq;
using System.Security.Cryptography;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;

namespace nettrash.Controllers
{
	[ApiController]
	[Route("[controller]")]
	public class MathController : ControllerBase
	{
		#region Private properties



		private readonly ILogger<MathController> _logger;



		#endregion
		#region Public constructors



		public MathController(ILogger<MathController> logger)
		{
			_logger = logger;
		}



		#endregion
		#region Public methods



		[HttpGet("guid")]
#pragma warning disable CA1720 // Identifier contains type name
		public Response.Guid Guid()
#pragma warning restore CA1720 // Identifier contains type name
		{
			return new Response.Guid { Value = System.Guid.NewGuid().ToString() };
		}

		[HttpGet("luhn")]
		public Response.Luhn Luhn(string source)
		{
			try
			{
				bool bResult = false;
				string sResult = string.Empty;
				if (source.Trim().All(char.IsDigit))
				{
					byte[] data = source.Trim().Select(c => byte.Parse(c.ToString())).ToArray();
					int value = 0;
					for (int i = 0; i < data.Length; i++)
					{
						if (i % 2 == 0)
						{
							int p = data[i] * 2;
							if (p > 9)
							{
								p -= 9;
							}

							value += p;
						}
						else
						{
							value += data[i];
						}
					}
					bResult = value % 10 == 0;
					sResult = bResult ? "valid" : "not valid";
				}
				else
				{
					sResult = "it's not a number";
				}
				return new Response.Luhn { Result = true, LuhnResult = bResult, ErrorText = sResult };
			}
			catch (Exception ex)
			{
				return new Response.Luhn { Result = false, LuhnResult = false, ErrorText = ex.Message };
			}
		}

		[HttpPost("hash")]
		public Response.Hash Hash(Request.Hash request)
		{
			try
			{
				var hash = HashAlgorithm.Create(request.Algorithm);
				byte[] hashValue = hash.ComputeHash(System.Text.Encoding.UTF8.GetBytes(request.SourceText));
				string sResult = string.Join("", hashValue.Select(b => b.ToString("X2")));
				return new Response.Hash { Result = true, Value = sResult };
			}
			catch (Exception ex)
			{
				return new Response.Hash { Result = false, Value = ex.Message };
			}

		}



		#endregion
	}
}